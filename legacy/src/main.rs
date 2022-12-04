/*
Advent of Code in Rust.

This program should be usable for any year of the Advent of Code
calendar challenges.

Author: Derek W.
*/

mod twentyone;
use clap::{App, Arg};

fn main() {
    let args = App::new("Advent of Code in Rust")
        .version("0,1")
        .author("Derek W.")
        .about("CLI for Advent of Code")
        .arg(
            Arg::with_name("path")
                .help("Path to data directory containing inputs")
                .short("path")
                .required(true)
                .value_name("path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("year")
                .help("Advent of Code year")
                .short("y")
                .required(true)
                .value_name("year")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("day")
                .help("Day with which to run")
                .short("d")
                .required(true)
                .value_name("day")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test")
                .help("Whether or not to use test")
                .short("t")
                .required(false)
                .value_name("test")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number")
                .help("Index number of the input files to be run. Defaults to 0.")
                .short("n")
                .required(false)
                .value_name("number")
                .takes_value(true),
        )
        .get_matches();

    // Parse the args to get the data we need
    let day: u8 = args
        .value_of("day")
        .unwrap_or("0")
        .trim()
        .parse()
        .expect("Day needs to be a number");
    let year: i64 = args
        .value_of("year")
        .unwrap_or("0")
        .trim()
        .parse()
        .expect("Year needs to be a number");
    let number: i64 = args
        .value_of("number")
        .unwrap_or("0")
        .trim()
        .parse()
        .expect("Number needs to be a number");
    let path = args.value_of("path").unwrap_or("").to_string();
    let testing = args.is_present("test");

    let mut file_suffix = "";
    if testing {
        file_suffix = ".test";
    }

    let input_path = format!("{}/{}/{}_{}{}.txt", path, year, day, number, file_suffix);

    println!("Processing: {}", input_path);

    match year {
        2021 => {
            twentyone::run(day, &input_path);
        }
        _ => {
            println!("No implementation for year {}", year);
        }
    }
}
