#[derive(Default)]
pub struct Mask {
  mask: u64,
  bits: u64
}

impl Mask {
  pub fn update(&mut self, mask: u64, bits: u64) {
    self.mask = mask;
    self.bits = bits;
  }

  pub fn apply_to(&self, value: u64) -> u64 {
    self.bits | self.mask & value
  }

  pub fn for_all_floating(&self, value: u64, mut f: impl FnMut(u64)) {
    fn rec(f: &mut impl FnMut(u64), i: u64, n: u64) {
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
