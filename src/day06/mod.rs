use std::io::Cursor;

use crate::utils::line_groups;

pub fn run(input: &str) -> (usize, usize) {
  let mut part1 = 0;
  let mut part2 = 0;

  for group in line_groups(Cursor::new(input)) {
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

    part1 += any.count_ones() as usize;
    part2 += all.count_ones() as usize;
  }

  (part1, part2)
}
