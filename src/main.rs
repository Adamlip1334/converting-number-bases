use clap::{App, Arg};
use std::process;

#[macro_use]
extern crate clap;
extern crate num_bigint;

use num_bigint::BigInt;

fn anybase_to_anybase(input_base: u32, output_base: u32, number: &str) -> Result<String, String> {
    // Validate the input number
    if !number.chars().all(|c| {
        c.is_digit(input_base) ||
        (input_base > 10 && c.is_alphabetic()) ||
        (input_base == 62 && c.is_ascii_punctuation()) ||
        (input_base == 64 && c.is_ascii_punctuation())
    }) {
        return Err(format!("Invalid number for base {}: {}", input_base, number));
    }

    let decimal = BigInt::parse_bytes(number.as_bytes(), input_base).unwrap();
    Ok(decimal.to_str_radix(output_base))
}


fn main() {
    let matches = App::new("Base Converter")
        .arg(
            Arg::with_name("input_base")
                .short("i")
                .long("input-base")
                .required(true)
                .takes_value(true)
                .help("The base of the input number"),
        )
        .arg(
            Arg::with_name("output_base")
                .short("o")
                .long("output-base")
                .required(true)
                .takes_value(true)
                .help("The base of the output number"),
        )
        .arg(
            Arg::with_name("number")
                .required(true)
                .help("The number to convert"),
        )
        .get_matches();

    let input_base = value_t!(matches, "input_base", u32).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let output_base = value_t!(matches, "output_base", u32).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let number = matches.value_of("number").unwrap();

    let result = anybase_to_anybase(input_base, output_base, number);
    match result {
        Ok(output) => println!("{} (base {}) = {} (base {})", number, input_base, output, output_base),
        Err(error) => println!("Error: {}", error),
    }

}
