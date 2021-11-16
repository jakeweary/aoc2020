use std::ops::RangeInclusive;

pub struct Rule<'a> {
  name: &'a str,
  ranges: [RangeInclusive<u32>; 2]
}

impl<'a> Rule<'a> {
  pub fn parse(input: &'a str) -> Option<Self> {
    let range = |input: &str| {
      let (min, max) = input.split_once('-')?;
      Some(min.parse().ok()?..=max.parse().ok()?)
    };

    let (name, ranges) = input.split_once(": ")?;
    let ranges = ranges.split_once(" or ")?;
    let ranges = [range(ranges.0)?, range(ranges.1)?];
    Some(Self { name, ranges })
  }

  pub fn check(&self, value: u32) -> bool {
    self.ranges.iter().any(|r| r.contains(&value))
  }
}
