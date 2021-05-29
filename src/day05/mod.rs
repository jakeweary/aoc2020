fn parse_seat_id(input: &str) -> Option<usize> {
  input.bytes()
    .map(|b| match b {
      b'F' | b'L' => Some(0),
      b'B' | b'R' => Some(1),
      _ => None
    })
    .try_fold(0, |n, bit| Some(n << 1 | bit?))
}

// https://math.stackexchange.com/a/1917515
fn sum_gauss(a: usize, b: usize) -> usize {
  (a + b) * (b - a + 1) / 2
}

pub fn run(input: &str) -> (usize, usize) {
  let (min, max, sum) = input.lines()
    .map(parse_seat_id)
    .try_fold((usize::MAX, 0, 0), |(min, max, sum), id| {
      id.map(|id| (min.min(id), max.max(id), sum + id))
    })
    .unwrap();

  let part1 = max;
  let part2 = sum_gauss(min, max) - sum;

  (part1, part2)
}
