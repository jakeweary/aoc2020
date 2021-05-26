use cell::*;
use field2d::*;

mod cell;
mod field2d;

fn part1(field: &Field2D<Cell>, cell: &Cell, pos: (i32, i32)) -> Cell {
  match cell {
    Floor => Floor,
    Empty => {
      let comfy = field.neighbors(pos)
        .all(|cell| *cell != Occupied);

      if comfy { Occupied } else { Empty }
    }
    Occupied => {
      let too_crowded = field.neighbors(pos)
        .filter(|cell| **cell == Occupied)
        .nth(3)
        .is_some();

      if too_crowded { Empty } else { Occupied }
    }
  }
}

fn part2(field: &Field2D<Cell>, cell: &Cell, pos: (i32, i32)) -> Cell {
  let mut visible_seats = DIRECTIONS.iter().filter_map(|&dir| {
    field.iter_dir(pos, dir).find(|cell| **cell != Floor)
  });

  match cell {
    Floor => Floor,
    Empty => {
      let comfy = visible_seats
        .all(|seat| *seat != Occupied);

      if comfy { Occupied } else { Empty }
    }
    Occupied => {
      let too_crowded = visible_seats
        .filter(|seat| **seat == Occupied)
        .nth(4)
        .is_some();

      if too_crowded { Empty } else { Occupied }
    }
  }
}

fn solve<F>(mut step: F, mut field: Field2D<Cell>) -> usize
where
  F: FnMut(&Field2D<Cell>, &Cell, (i32, i32)) -> Cell
{
  loop {
    let mut mutated_cells = 0;

    field.step(|field, before, pos| {
      let after = step(field, before, pos);
      if *before != after {
        mutated_cells += 1;
      }
      after
    });

    if mutated_cells == 0 {
      return field.cells.iter()
        .filter(|cell| **cell == Occupied)
        .count();
    }
  }
}

pub fn run(input: &str) -> (usize, usize) {
  let input = input.lines().map(|s| s.bytes().map(Cell::from));
  let field = Field2D::new(input);

  let part1 = solve(part1, field.clone());
  let part2 = solve(part2, field);

  (part1, part2)
}
