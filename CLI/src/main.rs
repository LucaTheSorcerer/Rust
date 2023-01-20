use clap::{App, Arg};
use std::io::{BufReader, Read};
use std::fs::File;

fn main() {
    let matches = App::new("read_file")
        .version("0.0.1")
        .author("Luca")
        .arg(Arg::with_name("input")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let file = File::open(input).unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
