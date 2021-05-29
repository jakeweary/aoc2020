pub const DIRECTIONS: [(i32, i32); 8] = [
  (-1, -1), ( 0, -1), ( 1, -1),
  (-1,  0),           ( 1,  0),
  (-1,  1), ( 0,  1), ( 1,  1)
];

#[derive(Clone)]
pub struct Grid<T> {
  pub cells: Vec<T>,
  pub width: i32,
  pub height: i32
}

impl<I, J, T> From<I> for Grid<T>
where
  I: IntoIterator<Item = J>,
  J: IntoIterator<Item = T>
{
  fn from(rows: I) -> Self {
    let mut cells = Vec::new();
    let mut height = 0;
    for row in rows {
      cells.extend(row);
      height += 1;
    }
    let width = cells.len() as i32 / height;
    Self { cells, width, height }
  }
}

impl<T> Grid<T> {
  pub fn step<F>(&mut self, mut f: F) -> Vec<T>
  where
    F: FnMut(&Self, &T, (i32, i32)) -> T
  {
    let xy = |i| (i % self.width, i / self.width);
    let new_cells = self.cells.iter().enumerate()
      .map(|(i, cell)| f(self, cell, xy(i as i32)))
      .collect();
    std::mem::replace(&mut self.cells, new_cells)
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
    DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
      self.at((x + dx, y + dy))
    })
  }

  pub fn iter_dir(&self, (x, y): (i32, i32), (dx, dy): (i32, i32)) -> impl Iterator<Item = &T> {
    (1..).map_while(move |steps| {
      self.at((x + steps * dx, y + steps * dy))
    })
  }
}
