use std::env;

use with_rust::day_1;
use with_rust::day_10;
use with_rust::day_11;
use with_rust::day_12;
use with_rust::day_13;
use with_rust::day_14;
use with_rust::day_15;
use with_rust::day_16;
use with_rust::day_17;
use with_rust::day_18;
use with_rust::day_19;
use with_rust::day_20;
use with_rust::day_21;
use with_rust::day_24;
use with_rust::day_4;
use with_rust::day_5;
use with_rust::day_6;
use with_rust::day_7;
use with_rust::day_8;
use with_rust::day_9;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => day_1::main(),
        "4" => day_4::main(),
        "5" => day_5::main(),
        "6" => day_6::main(),
        "7" => day_7::main(),
        "8" => day_8::main(),
        "9" => day_9::main(),
        "10" => day_10::main(),
        "11" => day_11::main(),
        "12" => day_12::main(),
        "13" => day_13::main(),
        "14" => day_14::main(),
        "15" => day_15::main(),
        "16" => day_16::main(),
        "17" => day_17::main(),
        "18" => day_18::main(),
        "19" => day_19::main(),
        "20" => day_20::main(),
        "21" => day_21::main(),
        "24" => day_24::main(),
        _ => println!("no tengo eso"),
    }
}
