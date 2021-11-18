use passport::Passport;

mod passport;

pub fn run(input: &str) -> (u32, u32) {
  let mut part1 = 0;
  let mut part2 = 0;

  for group in input.split("\n\n") {
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
