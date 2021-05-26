use ship::*;

mod ship;

pub fn run(input: &str) -> (usize, usize) {
  let actions = input.lines()
    .map(Action::from)
    .collect::<Vec<_>>();

  let part1 = {
    let mut ship = Ship::new(1, 0);
    ship.steer(&Mode::Standard, &actions);
    ship.distance_traveled() as usize
  };

  let part2 = {
    let mut ship = Ship::new(10, 1);
    ship.steer(&Mode::Waypoint, &actions);
    ship.distance_traveled() as usize
  };

  (part1, part2)
}
