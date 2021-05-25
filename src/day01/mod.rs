use std::str::FromStr;

pub fn run(input: &str) -> (usize, usize) {
  let nums = input.lines()
    .map(FromStr::from_str)
    .collect::<Result<Vec<usize>, _>>()
    .unwrap();

  let part1 = combinations!(nums a b)
    .find(|&(a, b)| a + b == 2020)
    .map(|(a, b)| a * b)
    .unwrap();

  let part2 = combinations!(nums a b c)
    .find(|&(a, b, c)| a + b + c == 2020)
    .map(|(a, b, c)| a * b * c)
    .unwrap();

  (part1, part2)
}
