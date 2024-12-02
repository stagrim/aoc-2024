use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let res = input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|xs| {
            let mut cmp = xs
                .windows(2)
                .map(|ws| (ws[0].abs_diff(ws[1]) <= 3).then_some(ws[0].cmp(&ws[1])));
            let first = cmp.next()?;
            if first == Some(std::cmp::Ordering::Equal) {
                None
            } else {
                cmp.all(|x| x == first).then_some(first?)
            }
        })
        .map(|x| if x.is_some() { 1 } else { 0 })
        .sum::<usize>();

    println!("{res:#?}");
}
