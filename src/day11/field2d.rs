pub const DIRECTIONS: [(i32, i32); 8] = [
  (-1, -1), ( 0, -1), ( 1, -1),
  (-1,  0),           ( 1,  0),
  (-1,  1), ( 0,  1), ( 1,  1)
];

#[derive(Clone)]
pub struct Field2D<T> {
  pub cells: Vec<T>,
  pub width: i32,
  pub height: i32
}

impl<T> Field2D<T> {
  pub fn new<I, J>(rows: I) -> Self
  where
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = T>
  {
    let mut cells = Vec::new();
    let mut height = 0;
    for row in rows {
      cells.extend(row);
      height += 1;
    }
    let width = cells.len() as i32 / height;
    Self { cells, width, height }
  }

  pub fn step<F>(&mut self, mut f: F)
  where
    F: FnMut(&Self, &T, (i32, i32)) -> T
  {
    let coords = |i| (i % self.width, i / self.width);
    self.cells = self.cells.iter().enumerate()
      .map(|(i, cell)| f(self, cell, coords(i as i32)))
      .collect();
  }

  pub fn at(&self, (x, y): (i32, i32)) -> Option<&T> {
    self.contains((x, y)).then(move || {
      &self.cells[(self.width * y + x) as usize]
    })
  }

  pub fn contains(&self, (x, y): (i32, i32)) -> bool {
    let Self { width: w, height: h, .. } = *self;
    (0..w).contains(&x) && (0..h).contains(&y)
  }

  pub fn neighbors(&self, (x, y): (i32, i32)) -> impl Iterator<Item = &T> {
    DIRECTIONS.iter().filter_map(move |&(xʹ, yʹ)| {
      self.at((x + xʹ, y + yʹ))
    })
  }

  pub fn iter_dir(&self, (x, y): (i32, i32), (xʹ, yʹ): (i32, i32)) -> impl Iterator<Item = &T> {
    (1..).map_while(move |steps| {
      self.at((x + steps * xʹ, y + steps * yʹ))
    })
  }
}
