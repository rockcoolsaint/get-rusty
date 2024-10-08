use std::fs::{self, File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problen creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    fn read_username_from_file() -> Result<String, io::Error> {
        // let mut s = String::new();
        // File::open("hello.txt")?.read_to_string(&mut s)?;
        // Ok(s)
        fs::read_to_string("hello.txt")
    }
}
