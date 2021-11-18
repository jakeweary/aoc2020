use std::str::FromStr;

fn part1(nums: &[u64]) -> Option<u64> {
  nums.iter().skip(25).enumerate().find_map(|(i, &n)| {
    let prev25 = &nums[i..][..25];
    combinations!(prev25 a b)
      .all(|(a, b)| a + b != n)
      .then(|| n)
  })
}

fn part2(nums: &[u64], target: u64) -> Option<u64> {
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

pub fn run(input: &str) -> (u64, u64) {
  let nums = input.lines()
    .map(FromStr::from_str)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

  let part1 = part1(&nums).unwrap();
  let part2 = part2(&nums, part1).unwrap();

  (part1, part2)
}
