use ship::*;

mod ship;

pub fn run(input: &str) -> (i32, i32) {
  let actions = input.lines()
    .map(Action::from)
    .collect::<Vec<_>>();

  let part1 = {
    let mut ship = Ship::new(1, 0);
    ship.navigate(&Mode::Standard, &actions);
    ship.distance_traveled()
  };

  let part2 = {
    let mut ship = Ship::new(10, 1);
    ship.navigate(&Mode::Waypoint, &actions);
    ship.distance_traveled()
  };

  (part1, part2)
}
