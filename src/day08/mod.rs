mod op;
mod program;

pub fn run(input: &str) -> (usize, usize) {
  let mut program = program::Program::parse(input).unwrap();

  let part1 = program.run().unwrap_err() as usize;
  let part2 = program.fix().unwrap() as usize;

  (part1, part2)
}
