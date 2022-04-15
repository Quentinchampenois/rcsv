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

fn valid_username(username: &str) -> bool {
    let invalid_chars = ["&", "[", "]", "(", ")", "%","^", "*", ";", ":", "\\", "/", "|", "%", "!"];

    for invalid_char in invalid_chars {
        if username.contains(invalid_char) {
            //println!("Invalid char: {:?} for username: {:?}", invalid_char, username);
            return false;
        }
    }


    return true;
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for (idx, result) in rdr.records().enumerate() {
        let csv_record = result?;
        let record: Record = csv_record.deserialize(None)?;
        //println!("{:?}", record.email);
        if !valid_username(&record.username) {
            println!("Line {:?} : Username {:?} is invalid", idx + 1, record.username)
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("Error while reading CSV: {}", err);
        process::exit(1);
    }
}