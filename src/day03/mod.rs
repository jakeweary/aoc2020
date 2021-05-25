use std::array::IntoIter;

fn count_trees(input: &str, slope_x: usize, slope_y: usize) -> usize {
  input.lines().step_by(slope_y)
    .zip((0..).step_by(slope_x))
    .map(|(line, pos)| line.as_bytes()[pos % line.len()])
    .filter(|&chr| chr == b'#')
    .count()
}

pub fn run(input: &str) -> (usize, usize) {
  let part1 = count_trees(input, 3, 1);
  let part2 = IntoIter::new([(1, 1), (5, 1), (7, 1), (1, 2)])
    .map(|(x, y)| count_trees(input, x, y))
    .fold(part1, |acc, n| acc * n);

  (part1, part2)
}
