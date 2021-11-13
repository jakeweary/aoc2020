#[derive(Default)]
pub struct Mask {
  mask: usize,
  bits: usize
}

impl Mask {
  pub fn update(&mut self, input: &str) {
    self.mask = input.bytes()
      .map(|c| match c { b'X' => 1, _ => 0 })
      .fold(0, |acc, bit| acc << 1 | bit);

    self.bits = input.bytes()
      .map(|c| match c { b'1' => 1, _ => 0 })
      .fold(0, |acc, bit| acc << 1 | bit);
  }

  pub fn apply(&self, value: usize) -> usize {
    self.bits | self.mask & value
  }

  pub fn floating(&self, value: usize, mut f: impl FnMut(usize)) {
    fn rec(f: &mut impl FnMut(usize), i: usize, n: usize) {
      match (i..36).find(|i| n >> i & 1 == 1) {
        Some(i) => {
          rec(f, i + 1, n);
          rec(f, i + 1, n ^ 1 << i);
        }
        None => f(n)
      }
    }

    let m = !self.mask & value | self.bits;
    rec(&mut |n| f(n | m), 0, self.mask);
  }
}
