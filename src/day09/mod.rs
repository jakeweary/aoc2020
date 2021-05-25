use std::str::FromStr;

fn part1(nums: &[usize]) -> Option<usize> {
  nums.iter().skip(25).enumerate().find_map(|(i, &n)| {
    let prev25 = &nums[i..][..25];
    combinations!(prev25 a b)
      .all(|(a, b)| a + b != n)
      .then(|| n)
  })
}

fn part2(nums: &[usize], target: usize) -> Option<usize> {
  for i in 0..nums.len() {
    let mut acc = 0;
    let nums = &nums[i..];
    for (i, n) in nums.iter().enumerate() {
      acc += n;
      if acc == target {
        let nums = &nums[..i + 1];
        let min = nums.iter().min()?;
        let max = nums.iter().max()?;
        return Some(min + max);
      }
      if acc > target {
        break;
      }
    }
  }
  None
}

pub fn run(input: &str) -> (usize, usize) {
  let nums = input.lines()
    .map(FromStr::from_str)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

  let part1 = part1(&nums).unwrap();
  let part2 = part2(&nums, part1).unwrap();

  (part1, part2)
}
