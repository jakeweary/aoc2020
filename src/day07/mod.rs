use std::collections::HashMap;

type Parsed<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

fn parse(input: &str) -> Option<Parsed<'_>> {
  input.lines().map(|line| {
    let mut hi = line.match_indices(" bag");
    let lo = line.match_indices(|c| matches!(c, '0'..='9'));
    let bag = &line[..hi.next()?.0];
    let contains = lo.zip(hi).map(|(lo, hi)| {
      let (qty, bag) = line[lo.0..hi.0].split_once(' ')?;
      Some((bag, qty.parse().ok()?))
    });
    Some((bag, contains.collect::<Option<_>>()?))
  }).collect()
}

fn part1(bag: &str, parsed: &mut Parsed<'_>) -> u32 {
  let mut count = 0;
  let mut stack = vec![bag];
  while let Some(bag) = stack.pop() {
    parsed.retain(|&k, v| {
      let found = v.iter().any(|&(k, _)| k == bag);
      if found {
        stack.push(k);
        count += 1;
      }
      !found
    });
  }
  count
}

fn part2(bag: &str, parsed: &Parsed<'_>) -> u32 {
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

pub fn run(input: &str) -> (u32, u32) {
  let mut parsed = parse(&input).unwrap();
  let part1 = part1("shiny gold", &mut parsed);
  let part2 = part2("shiny gold", &parsed);

  (part1, part2)
}
