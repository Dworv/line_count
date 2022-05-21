use std::env::args;
use std::error::Error;
use std::fs;

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
    let file = fs::read_to_string(&filename)?;
    let lines: Vec<&str> = file.lines().collect();
    let line_num = lines.len();
    let plural: &str;
    if line_num == 1 {
        plural = "";
    } else {plural = "s"}

    println!("{} has {} line{}", filename, line_num, plural);

    Ok(())
}
