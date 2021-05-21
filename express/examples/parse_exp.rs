use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

use clap::{Arg, App};
use express::{strip_comments_and_lower, syntax};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("parse_exp")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Parses an EXPRESS file")
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");


    let mut f = File::open(input).expect("file opens");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("read ok");

    let start = SystemTime::now();
    let s = strip_comments_and_lower(&buffer);
    println!("{}",s);
    let parsed = syntax(&s);
    if let Err(e) = parsed {
        eprintln!("Got err {:?}", e);
    }

    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    eprintln!("time {:?}", since_the_epoch);
    Ok(())
}
