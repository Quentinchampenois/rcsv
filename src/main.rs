use std::error::Error;
use std::io;
use std::process;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Record {
    email: String,
    username: String
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let csv_record = result?;
        let record: Record = csv_record.deserialize(None)?;
        println!("{:?}", record.email);
        println!("{:?}", record.username);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("Error while reading CSV: {}", err);
        process::exit(1);
    }
}