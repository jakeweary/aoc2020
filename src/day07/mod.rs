use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Parsed<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn parse(input: &str) -> Option<Parsed> {
  let mut map = HashMap::new();
  for line in input.lines() {
    let mut contains = Vec::new();
    let mut hi = line.match_indices(" bag");
    let lo = line.match_indices(|c| matches!(c, '0'..='9'));
    let bag = &line[..hi.next()?.0];
    for (lo, hi) in lo.zip(hi) {
      let (qty, bag) = line[lo.0..hi.0].split_once(' ')?;
      contains.push((bag, qty.parse().ok()?));
    }
    map.insert(bag, contains);
  }
  Some(map)
}

fn part1(parsed: &mut Parsed, bag: &str) -> usize {
  let mut count = 0;
  let mut stack = vec![bag];
  while let Some(bag) = stack.pop() {
    parsed.retain(|&k, v| {
      match v.iter().all(|&(k, _)| k != bag) {
        true => true,
        false => {
          stack.push(k);
          count += 1;
          false
        }
      }
    });
  }
  count
}

fn part2(parsed: &Parsed, bag: &str) -> usize {
  let mut count = 0;
  let mut stack = vec![(bag, 1)];
  while let Some((bag, qty1)) = stack.pop() {
    for &(bag, qty2) in &parsed[bag] {
      stack.push((bag, qty1 * qty2));
    }
    count += qty1;
  }
  count - 1
}

pub fn run(mut file: File) -> (usize, usize) {
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();

  let mut parsed = parse(&input).unwrap();
  let part1 = part1(&mut parsed, "shiny gold");
  let part2 = part2(&parsed, "shiny gold");

  (part1, part2)
}
