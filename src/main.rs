use clap::{App, Arg};
use std::process;

#[macro_use]
extern crate clap;


fn base_to_decimal(base: u32, number: &str) -> u32 {
    u32::from_str_radix(number, base).unwrap()
}

fn main() {
    let matches = App::new("Base Converter")
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .required(true)
                .takes_value(true)
                .help("The base of the number"),
        )
        .arg(
            Arg::with_name("number")
                .required(true)
                .help("The number to convert"),
        )
        .get_matches();

    let base = value_t!(matches, "base", u32).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let number = matches.value_of("number").unwrap();

    let decimal = base_to_decimal(base, number);
    println!("{} (base {}) = {} (base 10)", number, base, decimal);
}
