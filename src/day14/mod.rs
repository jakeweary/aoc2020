use std::collections::HashMap;

use self::mask::*;
use self::op::*;

mod mask;
mod op;

pub fn run(input: &str) -> (u64, u64) {
  let mut current_mask = Mask::default();
  let mut memory_part1 = HashMap::new();
  let mut memory_part2 = HashMap::new();

  for op in input.lines().filter_map(Op::parse) {
    match op {
      Op::Mask { mask, bits } => {
        current_mask.update(mask, bits);
      }
      Op::Write { addr, value } => {
        memory_part1.insert(addr, {
          current_mask.apply_to(value)
        });
        current_mask.for_all_floating(addr, |addr| {
          memory_part2.insert(addr, value);
        });
      }
    }
  }

  let part1 = memory_part1.values().sum();
  let part2 = memory_part2.values().sum();
  (part1, part2)
}
