fn parse_seat_id(input: &str) -> Option<u32> {
  input.bytes()
    .map(|b| match b {
      b'F' | b'L' => Some(0),
      b'B' | b'R' => Some(1),
      _ => None
    })
    .try_fold(0, |n, bit| Some(n << 1 | bit?))
}

// https://math.stackexchange.com/a/1917515
fn sum_gauss(a: u32, b: u32) -> u32 {
  (a + b) * (b - a + 1) / 2
}

pub fn run(input: &str) -> (u32, u32) {
  let (min, max, sum) = input.lines()
    .map(parse_seat_id)
    .try_fold((u32::MAX, 0, 0), |(min, max, sum), id| {
      id.map(|id| (min.min(id), max.max(id), sum + id))
    })
    .unwrap();

  let part1 = max;
  let part2 = sum_gauss(min, max) - sum;

  (part1, part2)
}
