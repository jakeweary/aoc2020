pub enum Op {
  Mask { mask: u64, bits: u64 },
  Write { addr: u64, value: u64 }
}

impl Op {
  pub fn parse(input: &str) -> Option<Self> {
    match input.split_once(" = ")? {
      ("mask", rhs) => {
        let mask = rhs.bytes()
          .map(|c| match c { b'X' => 1, _ => 0 })
          .fold(0, |acc, bit| acc << 1 | bit);

        let bits = rhs.bytes()
          .map(|c| match c { b'1' => 1, _ => 0 })
          .fold(0, |acc, bit| acc << 1 | bit);

        Some(Self::Mask { mask, bits })
      },
      (lhs, rhs) => {
        let not_digit = |c: char| !c.is_ascii_digit();
        let addr = lhs.trim_matches(not_digit).parse().ok()?;
        let value = rhs.parse().ok()?;

        Some(Self::Write { addr, value })
      }
    }
  }
}
