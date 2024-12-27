use clap::{Arg, Command};

mod examples;
pub mod utils;

use examples::example1;
use examples::example10;
use examples::example11;
use examples::example12;
use examples::example13;
use examples::example14;
use examples::example15;
use examples::example2;
use examples::example3;
use examples::example4;
use examples::example5;
use examples::example6;
use examples::example7;
use examples::example8;
use examples::example9;

fn main() {
    let matches = Command::new("Advent of Code")
        .version("1.0")
        .author("Vikram Saraph <vikram.saraph.22@gmail.com>")
        .about("Solutions for Advent of Code 2024")
        .arg(
            Arg::new("example")
                .short('e')
                .long("example")
                .help("Specifies which example to run")
                .required(true),
        )
        .get_matches();

    let example = matches.get_one::<String>("example").unwrap().as_str();

    match example {
        "1_1" => {
            example1::run1();
        }
        "1_2" => {
            example1::run2();
        }
        "2_1" => {
            example2::run1();
        }
        "2_2" => {
            example2::run2();
        }
        "3_1" => {
            example3::run1();
        }
        "3_2" => {
            example3::run2();
        }
        "4_1" => {
            example4::run1();
        }
        "4_2" => {
            example4::run2();
        }
        "5_1" => {
            example5::run1();
        }
        "5_2" => {
            example5::run2();
        }
        "6_1" => {
            example6::run1();
        }
        "7_1" => {
            example7::run1();
        }
        "7_2" => {
            example7::run2();
        }
        "8_1" => {
            example8::run1();
        }
        "8_2" => {
            example8::run2();
        }
        "9_1" => {
            example9::run1();
        }
        "9_2" => {
            example9::run2();
        }
        "10_1" => {
            example10::run1();
        }
        "10_2" => {
            example10::run2();
        }
        "11_1" => {
            example11::run1();
        }
        "12_1" => {
            example12::run1();
        }
        "12_2" => {
            example12::run2();
        }
        "13_1" => {
            example13::run1();
        }
        "13_2" => {
            example13::run2();
        }
        "14_1" => {
            example14::run1();
        }
        "14_2" => {
            example14::run2();
        }
        "15_1" => {
            example15::run1();
        }
        _ => {
            println!("Unknown example: {}", example);
        }
    }
}
