fn parse(line: &str) -> Option<(usize, usize, u8, &[u8])> {
  let mut parts = line.split(['-', ' ', ':'].as_ref());
  let num1 = parts.next()?.parse().ok()?;
  let num2 = parts.next()?.parse().ok()?;
  let chr = parts.next()?.bytes().next()?;
  let pwd = parts.nth(1)?.as_bytes();
  Some((num1, num2, chr, pwd))
}

pub fn run(input: &str) -> (usize, usize) {
  let parsed = input.lines()
    .map(parse)
    .collect::<Option<Vec<_>>>()
    .unwrap();

  let part1 = parsed.iter()
    .filter(|&&(lo, hi, chr, pwd)| {
      let count = pwd.iter().filter(|&&c| c == chr).count();
      (lo..=hi).contains(&count)
    })
    .count();

  let part2 = parsed.iter()
    .filter_map(|&(a, b, chr, pwd)| {
      let a = chr == pwd[a - 1];
      let b = chr == pwd[b - 1];
      (a ^ b).then(|| ())
    })
    .count();

  (part1, part2)
}
