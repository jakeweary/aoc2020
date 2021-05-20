use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(line: &str) -> (usize, usize, char, &str) {
  let inner = move || {
    let mut parts = line.split(['-', ' ', ':'].as_ref());
    let num1 = parts.next()?.parse().ok()?;
    let num2 = parts.next()?.parse().ok()?;
    let chr = parts.next()?.chars().next()?;
    let pwd = parts.nth(1)?;
    Some((num1, num2, chr, pwd))
  };
  inner().expect("Jarvis, run git blame")
}

pub fn run(input: File) -> (usize, usize) {
  let input = BufReader::new(input);
  let lines = input.lines().map(Result::unwrap).collect::<Vec<_>>();
  let parsed = lines.iter().map(|line| parse(line)).collect::<Vec<_>>();

  let part1 = parsed.iter()
    .filter(|&&(lo, hi, chr, pwd)| {
      let count = pwd.chars().filter(|&c| c == chr).count();
      (lo..=hi).contains(&count)
    })
    .count();

  let part2 = parsed.iter()
    .filter_map(|&(a, b, chr, pwd)| {
      let a = chr == pwd.chars().nth(a - 1)?;
      let b = chr == pwd.chars().nth(b - 1)?;
      (a ^ b).then(|| ())
    })
    .count();

  (part1, part2)
}
