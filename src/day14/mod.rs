use std::collections::HashMap;

mod mask;

pub fn run(input: &str) -> (usize, usize) {
  let part1 = part1(input).unwrap();
  let part2 = part2(input).unwrap();
  (part1, part2)
}

fn part1(input: &str) -> Option<usize> {
  let mut memory = vec![0; 1<<16];
  let mut mask = mask::Mask::default();

  for line in input.lines() {
    match line.split_once(" = ")? {
      ("mask", rhs) => mask.update(rhs),
      (lhs, rhs) => {
        let not_digit = |c: char| !c.is_ascii_digit();
        let addr = lhs.trim_matches(not_digit).parse::<usize>().ok()?;
        let value = rhs.parse().ok()?;
        memory[addr] = mask.apply(value);
      }
    }
  }

  Some(memory.iter().sum())
}

fn part2(input: &str) -> Option<usize> {
  let mut memory = HashMap::new();
  let mut mask = mask::Mask::default();

  for line in input.lines() {
    match line.split_once(" = ")? {
      ("mask", rhs) => mask.update(rhs),
      (lhs, rhs) => {
        let not_digit = |c: char| !c.is_ascii_digit();
        let addr = lhs.trim_matches(not_digit).parse::<usize>().ok()?;
        let value = rhs.parse().ok()?;
        mask.floating(addr, |addr| {
          memory.insert(addr, value);
        });
      }
    }
  }

  Some(memory.values().sum())
}
