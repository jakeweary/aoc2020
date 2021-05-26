use Action::*;
use Mode::*;

pub enum Mode {
  Standard,
  Waypoint
}

pub enum Action {
  N(i32),
  S(i32),
  E(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32)
}

impl From<&str> for Action {
  fn from(input: &str) -> Self {
    let (action, value) = input.split_at(1);
    let action = action.bytes().next().unwrap();
    let value = value.parse().unwrap();
    match action {
      b'N' => N(value),
      b'S' => S(value),
      b'E' => E(value),
      b'W' => W(value),
      b'L' => L(value),
      b'R' => R(value),
      b'F' => F(value),
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

  pub fn steer<'a, I>(&mut self, mode: &Mode, actions: I)
  where
    I: IntoIterator<Item = &'a Action>
  {
    for action in actions {
      match (action, mode) {
        (N(steps), Standard) => self.pos.north += steps,
        (S(steps), Standard) => self.pos.north -= steps,
        (E(steps), Standard) => self.pos.east += steps,
        (W(steps), Standard) => self.pos.east -= steps,

        (N(steps), Waypoint) => self.dir.north += steps,
        (S(steps), Waypoint) => self.dir.north -= steps,
        (E(steps), Waypoint) => self.dir.east += steps,
        (W(steps), Waypoint) => self.dir.east -= steps,

        (L(90) | R(270), _) => self.dir.rotate_left(),
        (L(270) | R(90), _) => self.dir.rotate_right(),
        (L(180) | R(180), _) => self.dir.inverse(),

        (F(steps), _) => {
          self.pos.east += steps * self.dir.east;
          self.pos.north += steps * self.dir.north;
        }

        _ => panic!()
      }
    }
  }
}
