pub fn run(input: &str) -> (i64, i64) {
  let mut lines = input.lines();
  let depart_time = lines.next().unwrap().parse::<i64>().unwrap();
  let buses = lines.next().unwrap();

  let buses = buses.split(',')
    .enumerate()
    .filter_map(|(i, id)| Some((i, id.parse().ok()?)))
    .map(|(i, id)| (id - i as i64, id))
    .collect::<Vec<_>>();

  let part1 = buses.iter()
    .map(|(_, bus)| (bus - depart_time % bus, bus))
    .reduce(|a, b| if a.0 < b.0 { a } else { b })
    .map(|(arrives_in, bus)| arrives_in * bus)
    .unwrap();

  let part2 = crt(&buses).unwrap();

  (part1, part2)
}

/// chinese remainder theorem
fn crt(pairs: &[(i64, i64)]) -> Option<i64> {
  let product = pairs.iter()
    .map(|&(_, m)| m)
    .product::<i64>();

  pairs.iter()
    .map(|&(r, m)| (r, m, product / m))
    .map(|(r, m, p)| Some(r * p * inv(p, m)?))
    .sum::<Option<i64>>()
    .map(|sum| sum % product)
}

/// modular multiplicative inverse
fn inv(x: i64, n: i64) -> Option<i64> {
  match gcd(x, n) {
    (1, x, _) => Some(x % n),
    _ => None
  }
}

/// greatest common divisor (extended euclidean algorithm)
fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
  match a {
    0 => (b, 0, 1),
    _ => {
      let (g, x, y) = gcd(b % a, a);
      (g, y - b / a * x, x)
    }
  }
}
