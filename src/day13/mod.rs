pub fn run(input: &str) -> (usize, usize) {
  let mut lines = input.lines();
  let depart_time = lines.next().unwrap().parse::<isize>().unwrap();
  let buses = lines.next().unwrap();

  let buses = buses.split(',').enumerate()
    .filter_map(|(i, id)| Some((i as isize, id.parse().ok()?)))
    .collect::<Vec<_>>();

  let part1 = buses.iter()
    .map(|(_, bus)| (bus, bus - depart_time % bus))
    .reduce(|a, b| if a.1 < b.1 { a } else { b })
    .map(|(bus, arrives_in)| bus * arrives_in)
    .unwrap() as usize;

  let part2 = {
    let (rs, ms) = buses.into_iter()
      .map(|(i, id)| (id - i, id))
      .unzip::<_, _, Vec<_>, Vec<_>>();

    crt(&rs, &ms).unwrap() as usize
  };

  (part1, part2)
}

/// greatest common divisor (extended euclidean algorithm)
fn gcd(a: isize, b: isize) -> (isize, isize, isize) {
  match a {
    0 => (b, 0, 1),
    _ => {
      let (g, x, y) = gcd(b.rem_euclid(a), a);
      (g, y - b.div_euclid(a) * x, x)
    }
  }
}

/// modular multiplicative inverse
fn mmi(x: isize, n: isize) -> Option<isize> {
  match gcd(x, n) {
    (1, x, _) => Some(x.rem_euclid(n)),
    _ => None
  }
}

/// chinese remainder theorem
fn crt(residues: &[isize], modulii: &[isize]) -> Option<isize> {
  let product = modulii.iter().product::<isize>();
  residues.iter().zip(modulii)
    .map(|(&r, &m)| (r, m, product / m))
    .try_fold(0, |acc, (r, m, p)| Some(acc + r * mmi(p, m)? * p))
    .map(|sum| sum % product)
}
