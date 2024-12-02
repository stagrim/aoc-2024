use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let numbers = input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res1 = numbers
        .iter()
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

    let res2 = numbers
        .iter()
        .map(|xs| {
            (-1..xs.len() as isize)
                .filter_map(|ignore_index| {
                    let binding = xs
                        .iter()
                        .enumerate()
                        .filter_map(|(i, v)| {
                            if ignore_index == i as isize {
                                None
                            } else {
                                Some(v)
                            }
                        })
                        .collect::<Vec<_>>();
                    let mut cmp = binding
                        .windows(2)
                        .map(|ws| (ws[0].abs_diff(*ws[1]) <= 3).then_some(ws[0].cmp(&ws[1])));

                    let first = cmp.next()?;
                    if first == Some(std::cmp::Ordering::Equal) {
                        None
                    } else {
                        cmp.all(|x| x == first).then_some(first?)
                    }
                })
                .next()
        })
        .map(|x| if x.is_some() { 1 } else { 0 })
        .sum::<usize>();

    println!("part1: {res1} part2: {res2}");
}
