use std::str::FromStr;

use super::rule::Rule;

pub struct Ticket {
  values: Vec<u32>
}

impl Ticket {
  pub fn parse(input: &str) -> Option<Self> {
    let values = input.split(',')
      .map(FromStr::from_str)
      .collect::<Result<_, _>>()
      .ok()?;

    Some(Self { values })
  }

  pub fn invalid_values<'a>(&'a self, rules: &'a [Rule<'_>]) -> impl Iterator<Item = &u32> {
    self.values.iter().filter(|&&value| {
      rules.iter().all(|rule| !rule.check(value))
    })
  }
}
