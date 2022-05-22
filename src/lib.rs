use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_args() -> Result<Vec<String>, &'static str> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        return Err("Invalid # of arguments supplied");
    }

    Ok(args)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = get_args()?;
    let filename = args[1].clone();
    let file = File::open(&filename)?;
    let mut line_num: u32 = 0;
    for _ in BufReader::new(file).lines() {
        line_num += 1;
    }
    let plural: &str;
    if line_num == 1 {
        plural = "";
    } else {plural = "s"}

    println!("{} has {} line{}", filename, line_num, plural);

    Ok(())
}
