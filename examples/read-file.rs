extern crate csv;
extern crate production_calendar_open_data;

use std::env;
use std::error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn run() -> Result<(), Box<error::Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<error::Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Ожидается 1 аргумент!")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}