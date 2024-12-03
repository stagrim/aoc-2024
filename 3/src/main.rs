use std::io::{stdin, Read};

use regex::Regex;

#[derive(Debug)]
enum Matched {
    Mul(isize),
    Do,
    DoNot,
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut enabled = true;
    let (sum1, sum2): (isize, isize) = Regex::new(r"(mul\([0-9]*,[0-9]*\))|do\(\)|don't\(\)")
        .unwrap()
        .find_iter(&input)
        .map(|e| match mul_parser::mul(e.as_str()).unwrap() {
            Matched::Mul(m) if enabled => (m, m),
            Matched::Mul(m) => (m, 0),
            Matched::Do => {
                enabled = true;
                (0, 0)
            }
            Matched::DoNot => {
                enabled = false;
                (0, 0)
            }
        })
        .fold((0, 0), |(a, b), (x, y)| (x + a, y + b));
    println!("sum1: {sum1}, sum2: {sum2}");
}

peg::parser! {
    grammar mul_parser() for str {
        rule number() -> isize
            = n:$(['0'..='9']+) {? n.parse().or(Err("isize")) }

        pub rule mul() -> Matched =
        precedence! {
            "mul(" x:number() "," y:number() ")" { Matched::Mul(x * y) }
            --
            "do()" { Matched::Do}
            "don't()" { Matched::DoNot}
        }
    }
}
