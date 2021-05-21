macro_rules! impl_with_fields(($($f:ident)+) => {
  #[derive(Default)]
  pub struct Passport<T> {
    $($f: T),+
  }

  impl<'a> Passport<Option<&'a str>> {
    pub fn parse(key_value_pairs: &'a str) -> Self {
      key_value_pairs
        .split_ascii_whitespace()
        .map(|pair| pair.split_once(':').unwrap())
        .fold(Self::default(), |acc, (field, value)| match field {
          $(stringify!($f) => Self { $f: Some(value), ..acc },)+
          _ => acc // ignore other fields
        })
    }

    pub fn complete(&self) -> Option<Passport<&'a str>> {
      match self {
        Self { $($f: Some($f)),+ } => Some(Passport { $($f),+ }),
        _ => None
      }
    }
  }
});

impl_with_fields!(byr iyr eyr hgt hcl ecl pid);

impl<'a> Passport<&'a str> {
  pub fn is_valid(&self) -> bool {
    let Self { byr, iyr, eyr, hgt, hcl, ecl, pid } = *self;
    let inner = move || {
      let byr = byr.parse().ok()?;
      let iyr = iyr.parse().ok()?;
      let eyr = eyr.parse().ok()?;
      let hgt = {
        let mid = hgt.find(|c| !matches!(c, '0'..='9'))?;
        let (hgt, units) = hgt.split_at(mid);
        (hgt.parse().ok()?, units)
      };
      let hcl = hcl.strip_prefix('#')?;
      Some({
        matches!((byr, iyr, eyr), (1920..=2002, 2010..=2020, 2020..=2030)) &&
        matches!(hgt, (150..=193, "cm") | (59..=76, "in")) &&
        matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") &&
        hcl.len() == 6 && hcl.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) &&
        pid.len() == 9 && pid.chars().all(|c| matches!(c, '0'..='9'))
      })
    };
    inner().unwrap_or(false)
  }
}
