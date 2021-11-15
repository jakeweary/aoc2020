use std::str::FromStr;

pub fn run(input: &str) -> (usize, usize) {
  let starting = input.trim().split(',')
    .map(FromStr::from_str)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

  let part1 = nth(&starting, 2020);
  let part2 = nth(&starting, 30000000);
  (part1, part2)
}

fn nth(starting: &[usize], n: usize) -> usize {
  let (&last, except_last) = starting.split_last().unwrap();

  let mut turns = vec![0; n];
  for (i, &n) in except_last.iter().enumerate() {
    turns[n] = 1 + i as u32;
  }

  (starting.len()..n).fold(last, |curr, turn| {
    match std::mem::replace(&mut turns[curr], turn as u32) {
      0 => 0,
      t => turn - t as usize
    }
  })
}
