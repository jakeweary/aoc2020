use std::fs::File;
use std::io::{BufRead, BufReader};

fn seat_id(input: &str) -> usize {
  input.chars()
    .map(|c| match c {
      'F' | 'L' => 0,
      'B' | 'R' => 1,
      _ => panic!("unexpected 5d chess")
    })
    .fold(0, |n, bit| n << 1 | bit)
}

pub fn run(input: File) -> (usize, usize) {
  let input = BufReader::new(input);
  let lines = input.lines().map(Result::unwrap);

  let mut ids = lines
    .map(|line| seat_id(&line))
    .collect::<Vec<_>>();

  ids.sort_unstable();

  let neighbors = ids.iter().zip(&ids[1..])
    .find(|&(a, b)| b - a == 2)
    .expect("somebody took my seat");

  let part1 = *ids.last().unwrap();
  let part2 = neighbors.0 + 1;

  (part1, part2)
}
