pub use Op::*;

pub enum Op {
  Acc(i32),
  Jmp(i32),
  Nop(i32)
}

impl Op {
  pub fn parse(input: &str) -> Option<Self> {
    let (op, arg) = input.split_once(' ')?;
    let arg = arg.parse().ok()?;
    match op {
      "acc" => Some(Acc(arg)),
      "jmp" => Some(Jmp(arg)),
      "nop" => Some(Nop(arg)),
      _ => None
    }
  }

  pub fn flip(&self) -> Option<Self> {
    match *self {
      Jmp(arg) => Some(Nop(arg)),
      Nop(arg) => Some(Jmp(arg)),
      _ => None
    }
  }
}
