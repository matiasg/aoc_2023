use std::env;

use with_rust::day_24;
use with_rust::day_4;
use with_rust::day_5;
use with_rust::day_6;
use with_rust::day_7;
use with_rust::day_8;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "4" => day_4::main(),
        "5" => day_5::main(),
        "6" => day_6::main(),
        "7" => day_7::main(),
        "8" => day_8::main(),
        "24" => day_24::main(),
        _ => println!("no tengo eso"),
    }
}
