pub fn run(input: &str) -> (u32, u32) {
  let mut part1 = 0;
  let mut part2 = 0;

  for group in input.split("\n\n") {
    let mut any = 0;
    let mut all = !0;

    for line in group.lines() {
      let answers = line.bytes()
        .map(|b| (b'a'..=b'z').contains(&b).then(|| b & 31))
        .try_fold(0u32, |n, b| Some(n | 1 << b?))
        .unwrap();

      any |= answers;
      all &= answers;
    }

    part1 += any.count_ones();
    part2 += all.count_ones();
  }

  (part1, part2)
}
