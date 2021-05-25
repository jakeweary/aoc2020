pub use Cell::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
  Floor,
  Empty,
  Occupied
}

impl From<u8> for Cell {
  fn from(input: u8) -> Self {
    match input {
      b'.' => Floor,
      b'L' => Empty,
      b'#' => Occupied,
      _ => panic!()
    }
  }
}
