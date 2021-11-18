use std::str::FromStr;

fn part1(joltages: &[u64]) -> u64 {
  let mut diff1 = 0;
  let mut diff3 = 0;
  for [lo, hi] in joltages.array_windows() {
    match hi - lo {
      1 => diff1 += 1,
      3 => diff3 += 1,
      _ => panic!()
    }
  }
  diff1 * diff3
}

fn part2(joltages: &[u64]) -> u64 {
  let mut ways = 1;
  let mut ones = 0;
  for [lo, hi] in joltages.array_windows() {
    match hi - lo {
      1 => ones += 1,
      3 => {
        // uhh.. tribonacci numbers?
        ways *= [1, 1, 2, 4, 7][ones];
        ones = 0;
      }
      _ => panic!()
    }
  }
  ways
}

pub fn run(input: &str) -> (u64, u64) {
  let mut joltages = input.lines()
    .map(FromStr::from_str)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

  joltages.push(0);
  joltages.sort_unstable();
  joltages.push(joltages.last().unwrap() + 3);

  let part1 = part1(&joltages);
  let part2 = part2(&joltages);

  (part1, part2)
}
