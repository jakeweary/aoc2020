fn parse_seat_id(input: &str) -> Option<usize> {
  input.bytes()
    .map(|b| match b {
      b'F' | b'L' => Some(0),
      b'B' | b'R' => Some(1),
      _ => None
    })
    .try_fold(0, |n, bit| Some(n << 1 | bit?))
}

pub fn run(input: &str) -> (usize, usize) {
  let mut ids = input.lines()
    .map(parse_seat_id)
    .collect::<Option<Vec<_>>>()
    .unwrap();

  ids.sort_unstable();

  let part1 = *ids.last().unwrap();
  let part2 = ids.array_windows()
    .find(|[a, b]| b - a == 2)
    .map(|[a, _]| a + 1)
    .unwrap();

  (part1, part2)
}
