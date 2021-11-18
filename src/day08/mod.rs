mod op;
mod program;

pub fn run(input: &str) -> (i32, i32) {
  let mut program = program::Program::parse(input).unwrap();

  let part1 = program.run().unwrap_err();
  let part2 = program.fix().unwrap();

  (part1, part2)
}
