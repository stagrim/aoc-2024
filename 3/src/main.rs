use std::io::{stdin, Read};

use regex::Regex;

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let sum: isize = Regex::new(r"mul\([0-9]*,[0-9]*\)")
        .unwrap()
        .find_iter(&input)
        .map(|e| mul_parser::mul(e.as_str()).unwrap())
        .sum();
    println!("sum: {sum}");
}

peg::parser! {
    grammar mul_parser() for str {
        rule number() -> isize
            = n:$(['0'..='9']+) {? n.parse().or(Err("isize")) }

        pub rule mul() -> isize
            = "mul(" x:number() "," y:number() ")" { x * y }
    }
}
