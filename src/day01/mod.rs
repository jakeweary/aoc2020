use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
mod macros;

pub fn run(input: File) -> (usize, usize) {
  let input = BufReader::new(input);
  let nums = input.lines()
    .map(|n| n.unwrap().parse().unwrap())
    .collect::<Vec<usize>>();

  let part1 = combinations!(nums a b)
    .find(|&(a, b)| a + b == 2020)
    .map(|(a, b)| a * b)
    .expect("how come?");

  let part2 = combinations!(nums a b c)
    .find(|&(a, b, c)| a + b + c == 2020)
    .map(|(a, b, c)| a * b * c)
    .expect("that doesn't add up");

  (part1, part2)
}
