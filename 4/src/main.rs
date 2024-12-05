use std::io::{stdin, Read};

use regex::Regex;

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let horizontal: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let vertical: Vec<String> = {
        let mut rows: Vec<_> = horizontal.iter().map(|l| l.chars()).collect();
        (0..horizontal[0].len())
            .map(|_col| {
                rows.iter_mut()
                    .map(|row| row.next().unwrap())
                    .collect::<String>()
            })
            .collect()
    };

    let (diagonal_cols, diagonal_rows) = (
        diagonal_strings(horizontal.iter()),
        diagonal_strings(horizontal.iter().rev()),
    );
    let xmas = Regex::new(r"XMAS").unwrap();
    let samx = Regex::new(r"SAMX").unwrap();

    let sum1: usize = vec![horizontal, vertical, diagonal_cols, diagonal_rows]
        .iter()
        .flat_map(|lines| {
            lines
                .iter()
                .map(|line| xmas.find_iter(&line).count() + samx.find_iter(&line).count())
        })
        .sum();

    // Part two, use only diagonal, get coordinates of matched A in MAS or SAM anf check all matched coordinates in the other diagonal
    println!("sum1: {sum1}");
}

fn diagonal_strings<'a, I>(lines: I) -> Vec<String>
where
    I: Iterator<Item = &'a String>,
{
    let mut rows: Vec<_> = lines.map(|l| l.chars()).collect();
    let a = (1..=rows.len()).map(|i| (0..i));
    let row_len = rows.len();
    let b = (1..rows.len()).rev().map(|i| ((row_len - i)..row_len));
    let ranges = a.chain(b);
    ranges
        .map(|c| c.filter_map(|_col| rows[_col].next()).collect::<String>())
        .collect()
}
