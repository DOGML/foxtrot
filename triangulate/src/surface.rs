use nalgebra_glm as glm;
use glm::{DVec2, DVec3, DVec4, DMat4};
use nurbs::BSplineSurface;
use crate::mesh::Vertex;

// Represents a surface in 3D space, with a function to project a 3D point
// on the surface down to a 2D space.
#[derive(Debug, Clone)]
pub enum Surface {
    Cylinder {
        location: DVec3,
        axis: DVec3,
        mat_i: DMat4,
        radius: f64,
        z_min: f64,
        z_max: f64,
    },
    Plane {
        normal: DVec3,
        mat_i: DMat4,
    },
    BSpline {
        surf: BSplineSurface,
    }
}

impl Surface {
    pub fn new_cylinder(axis: DVec3, ref_direction: DVec3, location: DVec3, radius: f64) -> Self {
        Surface::Cylinder {
            mat_i: Self::make_rigid_transform(axis, ref_direction, location)
                .try_inverse()
                .expect("Could not invert"),
            axis, radius, location,
            z_min: 0.0,
            z_max: 0.0,
        }
    }

    pub fn new_plane(axis: DVec3, ref_direction: DVec3, location: DVec3) -> Self {
        Surface::Plane {
            mat_i: Self::make_rigid_transform(axis, ref_direction, location)
                .try_inverse()
                .expect("Could not invert"),
            normal: axis,
        }
    }

    pub fn new_bspline(surf: BSplineSurface) -> Self
    {
        Surface::BSpline {surf }
    }

    pub fn make_affine_transform(z_world: DVec3, x_world: DVec3, y_world: DVec3, origin_world: DVec3) -> DMat4 {
        let mut mat = DMat4::identity();
        mat.set_column(0, &glm::vec3_to_vec4(&x_world));
        mat.set_column(1, &glm::vec3_to_vec4(&y_world));
        mat.set_column(2, &glm::vec3_to_vec4(&z_world));
        mat.set_column(3, &glm::vec3_to_vec4(&origin_world));
        *mat.get_mut((3, 3)).unwrap() =  1.0;
        mat
    }

    fn make_rigid_transform(z_world: DVec3, x_world: DVec3, origin_world: DVec3) -> DMat4 {
        let mut mat = DMat4::identity();
        mat.set_column(0, &glm::vec3_to_vec4(&x_world));
        mat.set_column(1, &glm::vec3_to_vec4(&z_world.cross(&x_world)));
        mat.set_column(2, &glm::vec3_to_vec4(&z_world));
        mat.set_column(3, &glm::vec3_to_vec4(&origin_world));
        *mat.get_mut((3, 3)).unwrap() =  1.0;
        mat
    }

    /// Lowers a 3D point on a specific surface into a 2D space defined by
    /// the surface type.  This should only be called from `lower_verts`,
    /// to ensure that `prepare` is called first.
    fn lower(&self, p: DVec3) -> DVec2 {
        let p = DVec4::new(p.x, p.y, p.z, 1.0);
        match self {
            Surface::Plane { mat_i, .. } => {
                glm::vec4_to_vec2(&(mat_i * p))
            },
            Surface::Cylinder { mat_i, z_min, z_max, .. } => {
                let p = mat_i * p;
                // We convert the Z coordinates to either add or subtract from
                // the radius, so that we maintain the right topology (instead
                // of doing something like theta-z coordinates, which wrap
                // around awkwardly).

                // Scale from radius=1 to radius=0.5 based on Z
                let z = (p.z - z_min) / (z_max - z_min);
                let scale = 1.0 / (1.0 + z);
                DVec2::new(p.x * scale, p.y * scale)
            },
            Surface::BSpline {surf } => {
                surf.uv_from_point(p.xyz())
            },
        }
    }

    fn prepare(&mut self, verts: &mut [Vertex]) {
        match self {
            Surface::Cylinder { mat_i, z_min, z_max, .. } => {
                *z_min = std::f64::INFINITY;
                *z_max = -std::f64::INFINITY;
                for v in verts {
                    let p = (*mat_i) * DVec4::new(v.pos.x, v.pos.y, v.pos.z, 1.0);
                    if p.z < *z_min {
                        *z_min = p.z;
                    }
                    if p.z > *z_max {
                        *z_max = p.z;
                    }
                }
            }
            _ => (),
        }
    }

    pub fn lower_verts(&mut self, verts: &mut [Vertex]) -> Vec<(f64, f64)> {
        self.prepare(verts);
        let mut pts = Vec::with_capacity(verts.len());
        for v in verts {
            // Project to the 2D subspace for triangulation
            let proj = self.lower(v.pos);
            // Update the surface normal
            v.norm = self.normal(v.pos, proj);
            pts.push((proj.x, proj.y));
        }
        pts
    }

    // Calculate the surface normal, using either the 3D or 2D position
    pub fn normal(&self, p: DVec3, uv: DVec2) -> DVec3 {
        match self {
            Surface::Plane { normal, .. } => *normal,
            Surface::Cylinder { location, axis, .. } => {
                // Project the point onto the axis
                let proj = (p - location).dot(axis);

                // Find the nearest point along the axis
                let nearest = location + axis * proj;

                // Then the normal is just pointing along that direction
                // (same hack as below)
                -(p - nearest).normalize()
            },
            Surface::BSpline { surf } => {
                // Calculate first order derivs, then cross them to get normal
                let derivs = surf.surface_derivs::<1>(uv);
                let n = derivs[1][0].cross(&derivs[0][1]);
                n.normalize()
            },
        }
    }

    pub fn sign(&self) -> bool {
        // TODO: this is a hack, why are cylinders different from planes?
        match self {
            Surface::Plane {..} => false,
            Surface::Cylinder {..} => true,
            Surface::BSpline {..} => true,
        }
    }
}
