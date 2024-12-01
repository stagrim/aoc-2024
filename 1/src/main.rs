use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let (mut left, mut right) = (vec![], vec![]);
    input.lines().into_iter().for_each(|s| {
        let mut xs = s.split("   ").map(|s| s.parse::<usize>().unwrap());
        left.push(xs.next().unwrap());
        right.push(xs.next().unwrap());
    });
    left.sort();
    right.sort();

    let sum = (0..left.len())
        .into_iter()
        .fold(0, |acc, i| acc + (left[i].abs_diff(right[i])));

    println!("part 1: {sum}");

    let mut freq = HashMap::new();

    right.iter().for_each(|i| {
        freq.entry(i)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });

    let sum: usize = left.iter().map(|i| i * freq.get(i).unwrap_or(&0)).sum();

    println!("part 2: {sum}");
}
