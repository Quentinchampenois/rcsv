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
            return false;
        }
    }


    return true;
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for (idx, result) in rdr.records().enumerate() {
        match result {
            Ok(csv_record) => {
                let record: Record = csv_record.deserialize(None)?;
                if !valid_username(&record.username) {
                    println!("Line {:?} : Username {:?} is invalid", idx + 1, record.username)
                }
            },
            Err(err) => {
                println!("Error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
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