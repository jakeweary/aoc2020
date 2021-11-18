use super::rule::Rule;

pub const FIELDS: usize = 20;

pub struct Ticket {
  values: [u32; FIELDS]
}

impl Ticket {
  pub fn parse(input: &str) -> Option<Self> {
    let mut values = [0; FIELDS];
    let parsed = input.split(',').map(str::parse);
    for (value, parsed) in values.iter_mut().zip(parsed) {
      *value = parsed.ok()?;
    }
    Some(Self { values })
  }

  pub fn field(&self, index: usize) -> u32 {
    self.values[index]
  }

  pub fn is_invalid(&self, rules: &[Rule<'_>]) -> bool {
    self.invalid_values(&rules).next().is_some()
  }

  pub fn invalid_values<'a>(&'a self, rules: &'a [Rule<'_>]) -> impl Iterator<Item = &u32> {
    self.values.iter().filter(|&&value| {
      rules.iter().all(|rule| !rule.matches(value))
    })
  }
}

impl IntoIterator for Ticket {
  type Item = u32;
  type IntoIter = std::array::IntoIter<u32, 20>;

  fn into_iter(self) -> Self::IntoIter {
    self.values.into_iter()
  }
}
