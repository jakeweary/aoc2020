pub use Op::*;

pub enum Op {
  Acc(isize),
  Jmp(isize),
  Nop(isize)
}

impl Op {
  pub fn parse(input: &str) -> Option<Self> {
    let (op, arg) = input.split_once(' ')?;
    match (op, arg.parse().ok()) {
      ("acc", arg) => arg.map(Acc),
      ("jmp", arg) => arg.map(Jmp),
      ("nop", arg) => arg.map(Nop),
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
