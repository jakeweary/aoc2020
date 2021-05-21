use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Parsed<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn parse(input: &str) -> Option<Parsed> {
  let mut map = HashMap::new();
  for line in input.lines() {
    let mut contains = Vec::new();
    let mut bags = line.match_indices(" bag");
    let nums = line.match_indices(|c| matches!(c, '0'..='9'));
    let bag = &line[..bags.next()?.0];
    for (lo, hi) in nums.zip(bags) {
      let (qty, bag) = line[lo.0..hi.0].split_once(' ')?;
      contains.push((qty.parse().ok()?, bag));
    }
    map.insert(bag, contains);
  }
  Some(map)
}

fn part1(parsed: &Parsed) -> usize {
  let mut visited = HashMap::new();
  let mut stack = vec!["shiny gold"];
  let mut count = 0;
  while let Some(bag) = stack.pop() {
    for (&k, v) in parsed {
      if v.iter().any(|&(_, v)| v == bag) {
        visited.entry(k).or_insert_with(|| {
          stack.push(k);
          count += 1;
        });
      }
    }
  }
  count
}

fn part2(parsed: &Parsed) -> usize {
  let mut stack = vec![(1, "shiny gold")];
  let mut count = 0;
  while let Some((qty1, bag)) = stack.pop() {
    for &(qty2, bag) in &parsed[bag] {
      stack.push((qty1 * qty2, bag));
    }
    count += qty1;
  }
  count - 1
}

pub fn run(mut file: File) -> (usize, usize) {
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();

  let parsed = parse(&input).unwrap();
  let part1 = part1(&parsed);
  let part2 = part2(&parsed);

  (part1, part2)
}
