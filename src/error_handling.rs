use std::fs;
use std::fs::File;
use std::io::{Error, Read};

pub fn main() {
    // let f = File::open("hello.txt");
    // let file = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem creating the file: {:?}", error),
    //         },
    //         other=> panic!("Problem opening the file {:?}", other),
    //     }
    // };

    // println!("{}", read_username_from_file().unwrap());
    println!("{}", last_char_of_first_line("hadsad\n123").unwrap());
}

fn read_username_from_file() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().skip(1).next()?.chars().last()
}