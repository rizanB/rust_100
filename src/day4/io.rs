// read input, write to/ read from file
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn main() {
    println!("enter your name:");

    let mut uname = String::new();

    // read input
    io::stdin()
        .read_line(&mut uname)
        .expect("failed to read line");

    println!("hello, {}!", uname.trim());

    // write output
    let mut file = File::create("output.txt").expect("could not create file");

    writeln!(file, "Hello, file!").expect("failed to write");

    // read from file
    let contents =
        fs::read_to_string("output.txt").expect("sth went wrong while reading this file");

    println!("file contains:\n{}", contents);
}
