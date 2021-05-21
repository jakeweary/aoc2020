use std::fs::File;

use passport::Passport;

use crate::utils::line_groups;

mod passport;

pub fn run(input: File) -> (usize, usize) {
  let mut part1 = 0;
  let mut part2 = 0;

  for group in line_groups(input) {
    let passport = Passport::parse(group.as_ref());
    if let Some(passport) = passport.complete() {
      part1 += 1;
      if passport.is_valid() {
        part2 += 1;
      }
    }
  }

  (part1, part2)
}
