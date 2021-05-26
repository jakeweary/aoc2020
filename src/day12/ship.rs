use Move::*;
use Rotate::*;
use Mode::*;

pub enum Mode {
  Standard,
  Waypoint
}

pub enum Move {
  North(i32),
  South(i32),
  East(i32),
  West(i32)
}

pub enum Rotate {
  Left(i32),
  Right(i32),
}

pub enum Action {
  Move(Move),
  Rotate(Rotate),
  Forward(i32),
}

impl From<&str> for Action {
  fn from(input: &str) -> Self {
    let (action, value) = input.split_at(1);
    let action = action.bytes().next().unwrap();
    let value = value.parse().unwrap();
    match action {
      b'N' => Self::Move(North(value)),
      b'S' => Self::Move(South(value)),
      b'E' => Self::Move(East(value)),
      b'W' => Self::Move(West(value)),
      b'L' => Self::Rotate(Left(value)),
      b'R' => Self::Rotate(Right(value)),
      b'F' => Self::Forward(value),
      _ => panic!()
    }
  }
}

struct Coords {
  east: i32,
  north: i32
}

impl Coords {
  fn rotate_left(&mut self) {
    let Self { east, north } = *self;
    self.east = -north;
    self.north = east;
  }

  fn rotate_right(&mut self) {
    let Self { east, north } = *self;
    self.east = north;
    self.north = -east;
  }

  fn inverse(&mut self) {
    self.east = -self.east;
    self.north = -self.north;
  }
}

pub struct Ship {
  dir: Coords,
  pos: Coords
}

impl Ship {
  pub fn new(east: i32, north: i32) -> Self {
    let dir = Coords { east, north };
    let pos = Coords { east: 0, north: 0 };
    Ship { dir, pos }
  }

  pub fn distance_traveled(&self) -> i32 {
    self.pos.east.abs() + self.pos.north.abs()
  }

  pub fn navigate<'a, I>(&mut self, mode: &Mode, actions: I)
  where
    I: IntoIterator<Item = &'a Action>
  {
    for action in actions {
      match action {
        Action::Move(m) => match mode {
          Standard => match m {
            North(steps) => self.pos.north += steps,
            South(steps) => self.pos.north -= steps,
            East(steps) => self.pos.east += steps,
            West(steps) => self.pos.east -= steps
          },
          Waypoint => match m {
            North(steps) => self.dir.north += steps,
            South(steps) => self.dir.north -= steps,
            East(steps) => self.dir.east += steps,
            West(steps) => self.dir.east -= steps
          }
        }
        Action::Rotate(r) => match r {
          Left(90) | Right(270) => self.dir.rotate_left(),
          Left(270) | Right(90) => self.dir.rotate_right(),
          Left(180) | Right(180) => self.dir.inverse(),
          _ => panic!()
        }
        Action::Forward(steps) => {
          self.pos.east += steps * self.dir.east;
          self.pos.north += steps * self.dir.north;
        }
      }
    }
  }
}

