pub struct Id(usize);
pub struct LengthMeasure(f64);
pub struct CountMeasure(f64);

////////////////////////////////////////////////////////////////////////////////

pub enum Entity<S> {
    AdvancedBrepShapeRepresentation(S, Id, Id),
    AdvancedFace(S, Vec<Id>, Id, bool),
    ApplicationContext(S),
    ApplicationProtocolDefinition(S, S, u32, Id),
    Axis2Placement3d(S, Id, Id, Id),
    CartesianPoint(S, (f64, f64, f64)),
    Circle(S, Id, f64),
    ClosedShell(S, Vec<Id>),
    ColourRgb(S, f64, f64, f64),
    CylindricalSurface(S, Id, f64),
    Direction(S, (f64, f64, f64)),
    EdgeCurve(S, Id, Id, Id, bool),
    EdgeLoop(S, Vec<Id>),
    FaceBound(S, Id, bool),
    FillAreaStyle(S, Id),
    FillAreaStyleColour(S, Id),
    Line(S, Id, Id),
    MechanicalDesignGeometricPresentationRepresentation(S, Id, Id),
    ManifoldSolidBrep(S, Id),
    OrientedEdge(S, Id, bool),
    Plane(S, Id),
    PresentationStyleAssignment(Id),
    Product(S, S, S, Id),
    ProductCategory(S, S),
    ProductContext(S, Id, S),
    ProductDefinition(S, S, Id, Id),
    ProductDefinitionContext(S, Id, S),
    ProductDefinitionFormationWithSpecifiedSource(S, S, Id),
    ProductDefinitionShape(S, S, Id),
    ProductRelatedProductCategory(S, S, Id),
    PropertyDefinition(S, S, Id),
    PropertyDefinitionRepresentation(Id, Id),
    Representation(S, Id, Id),
    ShapeDefinitionRepresentation(Id, Id),
    ShapeRepresentation(S, Id, Id),
    ShapeRepresentationRelationship(S, S, Id, Id),
    StyledItem(S, Id, Id),
    SurfaceStyleUsage(Id),
    SurfaceSideStyle(S, Id),
    SurfaceStyleFillArea(Id),
    UncertaintyMeasureWithUnit(LengthMeasure, Id, S, S),

    ValueRepresentationItem(S,CountMeasure),
    Vector(S, Id, f64),
    VertexPoint(S, Id),

    ComplexEntity, // lol we're not handling these
}

pub fn parse(_data: &[u8]) -> Vec<Entity<&'static str>> {
    use Entity::*;
    vec![
        PropertyDefinitionRepresentation(Id(14),Id(12)),
        PropertyDefinitionRepresentation(Id(15),Id(13)),
        Representation("",Id(16),Id(219)),
        Representation("",Id(17),Id(219)),
        PropertyDefinition("pmivalidationproperty","",Id(224)),
        PropertyDefinition("pmivalidationproperty","",Id(224)),
        ValueRepresentationItem("numberofannotations",CountMeasure(0.)),
        ValueRepresentationItem("numberofviews",CountMeasure(0.)),
        ShapeRepresentationRelationship("","",Id(143),Id(19)),
        AdvancedBrepShapeRepresentation("",Id(141),Id(219)),
        Circle("",Id(150),0.00635),
        Circle("",Id(151),0.00635),
        CylindricalSurface("",Id(149),0.00635),
        OrientedEdge("",Id(51),false),
        OrientedEdge("",Id(52),false),
        OrientedEdge("",Id(53),true),
        OrientedEdge("",Id(54),true),
        OrientedEdge("",Id(55),true),
        OrientedEdge("",Id(56),false),
        OrientedEdge("",Id(57),false),
        OrientedEdge("",Id(52),true),
        OrientedEdge("",Id(58),true),
        OrientedEdge("",Id(59),false),
        OrientedEdge("",Id(60),false),
        OrientedEdge("",Id(56),true),
        OrientedEdge("",Id(61),false),
        OrientedEdge("",Id(54),false),
        OrientedEdge("",Id(62),true),
        OrientedEdge("",Id(59),true),
        OrientedEdge("",Id(63),true),
        OrientedEdge("",Id(64),false),
        OrientedEdge("",Id(53),false),
        OrientedEdge("",Id(57),true),
        OrientedEdge("",Id(60),true),
        OrientedEdge("",Id(62),false),
        OrientedEdge("",Id(63),false),
        OrientedEdge("",Id(51),true),
        OrientedEdge("",Id(61),true),
        OrientedEdge("",Id(58),false),
        OrientedEdge("",Id(55),false),
        OrientedEdge("",Id(64),true),
        EdgeCurve("",Id(65),Id(66),Id(75),true),
        EdgeCurve("",Id(67),Id(65),Id(76),true),
        EdgeCurve("",Id(67),Id(68),Id(77),true),
        EdgeCurve("",Id(68),Id(66),Id(78),true),
        EdgeCurve("",Id(65),Id(69),Id(79),true),
        EdgeCurve("",Id(70),Id(69),Id(80),true),
        EdgeCurve("",Id(67),Id(70),Id(81),true),
        EdgeCurve("",Id(69),Id(71),Id(82),true),
        EdgeCurve("",Id(72),Id(71),Id(83),true),
        EdgeCurve("",Id(70),Id(72),Id(84),true),
        EdgeCurve("",Id(66),Id(71),Id(85),true),
        EdgeCurve("",Id(68),Id(72),Id(86),true),
        EdgeCurve("",Id(73),Id(73),Id(20),true),
        EdgeCurve("",Id(74),Id(74),Id(21),true),
        VertexPoint("",Id(189)),
        VertexPoint("",Id(190)),
        VertexPoint("",Id(192)),
        VertexPoint("",Id(194)),
        VertexPoint("",Id(198)),
        VertexPoint("",Id(200)),
        VertexPoint("",Id(204)),
        VertexPoint("",Id(206)),
        VertexPoint("",Id(213)),
        VertexPoint("",Id(215)),
        Line("",Id(188),Id(87)),
        Line("",Id(191),Id(88)),
        Line("",Id(193),Id(89)),
        Line("",Id(195),Id(90)),
        Line("",Id(197),Id(91)),
        Line("",Id(199),Id(92)),
        Line("",Id(201),Id(93)),
        Line("",Id(203),Id(94)),
        Line("",Id(205),Id(95)),
        Line("",Id(207),Id(96)),
        Line("",Id(209),Id(97)),
        Line("",Id(210),Id(98)),
        Vector("",Id(158),1.),
        Vector("",Id(159),1.),
        Vector("",Id(160),1.),
        Vector("",Id(161),1.),
        Vector("",Id(164),1.),
        Vector("",Id(165),1.),
        Vector("",Id(166),1.),
        Vector("",Id(169),1.),
        Vector("",Id(170),1.),
        Vector("",Id(171),1.),
        Vector("",Id(174),1.),
        Vector("",Id(175),1.),
        EdgeLoop("",vec![Id(23),Id(24),Id(25),Id(26)]),
        EdgeLoop("",vec![Id(27),Id(28),Id(29),Id(30)]),
        EdgeLoop("",vec![Id(31),Id(32),Id(33),Id(34)]),
        EdgeLoop("",vec![Id(35),Id(36),Id(37),Id(38)]),
        EdgeLoop("",vec![Id(39)]),
        EdgeLoop("",vec![Id(40)]),
        EdgeLoop("",vec![Id(41),Id(42),Id(43),Id(44)]),
        EdgeLoop("",vec![Id(45)]),
        EdgeLoop("",vec![Id(46),Id(47),Id(48),Id(49)]),
        EdgeLoop("",vec![Id(50)]),
        FaceBound("",Id(99),true),
        FaceBound("",Id(100),true),
        FaceBound("",Id(101),true),
        FaceBound("",Id(102),true),
        FaceBound("",Id(103),true),
        FaceBound("",Id(104),true),
        FaceBound("",Id(105),true),
        FaceBound("",Id(106),true),
        FaceBound("",Id(107),true),
        FaceBound("",Id(108),true),
        Plane("",Id(145)),
        Plane("",Id(146)),
        Plane("",Id(147)),
        Plane("",Id(148)),
        Plane("",Id(152)),
        Plane("",Id(153)),
        AdvancedFace("",vec![Id(109)],Id(119),true),
        AdvancedFace("",vec![Id(110)],Id(120),false),
        AdvancedFace("",vec![Id(111)],Id(121),false),
        AdvancedFace("",vec![Id(112)],Id(122),true),
        AdvancedFace("",vec![Id(113),Id(114)],Id(22),false),
        AdvancedFace("",vec![Id(115),Id(116)],Id(123),true),
        AdvancedFace("",vec![Id(117),Id(118)],Id(124),false),
        ClosedShell("",vec![Id(125),Id(126),Id(127),Id(128),Id(129),Id(130),Id(131)]),
        StyledItem("",Id(134),Id(141)),
        PresentationStyleAssignment(Id(135)),
        SurfaceStyleUsage(Id(136)),
        SurfaceSideStyle("",Id(137)),
        SurfaceStyleFillArea(Id(138)),
        FillAreaStyle("",Id(139)),
        FillAreaStyleColour("",Id(140)),
        ColourRgb("",0.615686274509804,0.811764705882353,0.929411764705882),
        ManifoldSolidBrep("Part1",Id(132)),
        ShapeDefinitionRepresentation(Id(224),Id(143)),
        ShapeRepresentation("Part1",Id(144),Id(219)),
        Axis2Placement3d("",Id(186),Id(154),Id(155)),
        Axis2Placement3d("",Id(187),Id(156),Id(157)),
        Axis2Placement3d("",Id(196),Id(162),Id(163)),
        Axis2Placement3d("",Id(202),Id(167),Id(168)),
        Axis2Placement3d("",Id(208),Id(172),Id(173)),
        Axis2Placement3d("",Id(211),Id(176),Id(177)),
        Axis2Placement3d("",Id(212),Id(178),Id(179)),
        Axis2Placement3d("",Id(214),Id(180),Id(181)),
        Axis2Placement3d("",Id(216),Id(182),Id(183)),
        Axis2Placement3d("",Id(217),Id(184),Id(185)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(-1.,0.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(0.,1.,0.)),
        Direction("",(0.,0.,-1.)),
        Direction("",(0.,1.,0.)),
        Direction("",(0.,0.,-1.)),
        Direction("",(0.,1.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(0.,0.,-1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(-1.,0.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(0.,1.,0.)),
        Direction("",(0.,0.,-1.)),
        Direction("",(0.,1.,0.)),
        Direction("",(0.,1.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(1.,0.,0.)),
        Direction("",(0.,0.,-1.)),
        Direction("",(-1.,0.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        Direction("",(0.,0.,1.)),
        Direction("",(1.,0.,0.)),
        CartesianPoint("",(0.,0.,0.)),
        CartesianPoint("",(0.,0.0127,0.0254)),
        CartesianPoint("",(0.,0.0127,0.)),
        CartesianPoint("",(0.,0.,0.)),
        CartesianPoint("",(-3.46944695195361E-18,0.0254,0.)),
        CartesianPoint("",(0.,0.,0.0254)),
        CartesianPoint("",(0.,0.,0.0254)),
        CartesianPoint("",(0.,0.0127,0.0254)),
        CartesianPoint("",(-3.46944695195361E-18,0.0254,0.0254)),
        CartesianPoint("",(-3.46944695195361E-18,0.0254,0.0254)),
        CartesianPoint("",(0.0254,0.,0.0254)),
        CartesianPoint("",(0.0254,0.,0.)),
        CartesianPoint("",(0.0508,0.,0.)),
        CartesianPoint("",(0.0508,0.,0.0254)),
        CartesianPoint("",(0.0508,0.,0.0254)),
        CartesianPoint("",(0.0254,0.,0.0254)),
        CartesianPoint("",(0.0508,0.0127,0.0254)),
        CartesianPoint("",(0.0508,0.0127,0.)),
        CartesianPoint("",(0.0508,0.0254,0.)),
        CartesianPoint("",(0.0508,0.0254,0.0254)),
        CartesianPoint("",(0.0508,0.0254,0.0254)),
        CartesianPoint("",(0.0508,0.0127,0.0254)),
        CartesianPoint("",(0.0254,0.0254,0.0254)),
        CartesianPoint("",(0.0254,0.0254,0.)),
        CartesianPoint("",(0.0254,0.0254,0.0254)),
        CartesianPoint("",(0.0254,0.0127,0.0254)),
        CartesianPoint("",(0.0254,0.0127,0.0254)),
        CartesianPoint("",(0.03175,0.0127,0.0254)),
        CartesianPoint("",(0.0254,0.0127,0.)),
        CartesianPoint("",(0.03175,0.0127,0.)),
        CartesianPoint("",(0.0254,0.0127,0.0254)),
        CartesianPoint("",(0.0254,0.0127,0.)),
        MechanicalDesignGeometricPresentationRepresentation("",Id(133),Id(219)),
        ComplexEntity,
        UncertaintyMeasureWithUnit(LengthMeasure(5.0E-6),Id(223),"DISTANCE_ACCURACY_VALUE","MaximumToleranceappliedtomodel"),
        ComplexEntity,
        ComplexEntity,
        ComplexEntity,
        ProductDefinitionShape("","",Id(225)),
        ProductDefinition("","",Id(227),Id(226)),
        ProductDefinitionContext("",Id(233),"design"),
        ProductDefinitionFormationWithSpecifiedSource("","",Id(229)),
        ProductRelatedProductCategory("","",Id(229)),
        Product("Part1","Part1","Part1",Id(231)),
        ProductCategory("",""),
        ProductContext("",Id(233),"mechanical"),
        ApplicationProtocolDefinition("internationalstandard","automotive_design",2010,Id(233)),
        ApplicationContext("coredataforautomotivemechanicaldesignprocesses"),
    ]
}