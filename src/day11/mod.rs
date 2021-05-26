use cell::*;
use grid::*;

mod cell;
mod grid;

fn part1(grid: &Grid<Cell>, cell: &Cell, pos: (i32, i32)) -> Cell {
  let comfy = ||
    grid.neighbors(pos)
      .all(|cell| *cell != Occupied);

  let too_crowded = ||
    grid.neighbors(pos)
      .filter(|cell| **cell == Occupied)
      .nth(3)
      .is_some();

  match *cell {
    Empty if comfy() => Occupied,
    Occupied if too_crowded() => Empty,
    cell => cell
  }
}

fn part2(grid: &Grid<Cell>, cell: &Cell, pos: (i32, i32)) -> Cell {
  let visible_seats = ||
    DIRECTIONS.iter().filter_map(|&dir| {
      grid.iter_dir(pos, dir).find(|cell| **cell != Floor)
    });

  let comfy = ||
    visible_seats()
      .all(|seat| *seat != Occupied);

  let too_crowded = ||
    visible_seats()
      .filter(|seat| **seat == Occupied)
      .nth(4)
      .is_some();

  match *cell {
    Empty if comfy() => Occupied,
    Occupied if too_crowded() => Empty,
    cell => cell
  }
}

fn solve<F>(mut step: F, mut grid: Grid<Cell>) -> usize
where
  F: FnMut(&Grid<Cell>, &Cell, (i32, i32)) -> Cell
{
  loop {
    let mut mutated_cells = 0;

    grid.step(|grid, before, pos| {
      let after = step(grid, before, pos);
      if *before != after {
        mutated_cells += 1;
      }
      after
    });

    if mutated_cells == 0 {
      return grid.cells.iter()
        .filter(|cell| **cell == Occupied)
        .count();
    }
  }
}

pub fn run(input: &str) -> (usize, usize) {
  let rows = input.lines().map(|row| row.bytes().map(Cell::from));
  let grid = Grid::new(rows);

  let part1 = solve(part1, grid.clone());
  let part2 = solve(part2, grid);

  (part1, part2)
}
