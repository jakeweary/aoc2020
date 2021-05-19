use std::array::IntoIter;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

fn count_trees(input: &mut BufReader<File>, slope_x: usize, slope_y: usize) -> usize {
  input.seek(SeekFrom::Start(0)).unwrap();
  input.lines().step_by(slope_y).map(Result::unwrap)
    .zip((0..).step_by(slope_x))
    .map(|(line, pos)| line.chars().nth(pos % line.len()))
    .filter(|chr| chr.unwrap() == '#')
    .count()
}

pub fn run(input: File) -> (usize, usize) {
  let input = &mut BufReader::new(input);

  let part1 = count_trees(input, 3, 1);
  let part2 = IntoIter::new([(1, 1), (5, 1), (7, 1), (1, 2)])
    .map(|(x, y)| count_trees(input, x, y))
    .fold(part1, |acc, n| acc * n);

  (part1, part2)
}
