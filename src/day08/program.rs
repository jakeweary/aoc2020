use super::op::*;

pub struct Program {
  ops: Vec<Op>
}

impl Program {
  pub fn parse(input: &str) -> Option<Program> {
    input.lines()
      .map(Op::parse)
      .collect::<Option<_>>()
      .map(|ops| Self { ops })
  }

  pub fn run(&self) -> Result<i32, i32> {
    let mut visited = vec![false; self.ops.len()];
    let mut cur = 0;
    let mut acc = 0;

    while let Some(op) = self.ops.get(cur as usize) {
      let visited = &mut visited[cur as usize];
      if *visited {
        return Err(acc);
      }

      *visited = true;
      cur += 1;
      match op {
        Acc(arg) => acc += arg,
        Jmp(arg) => cur += arg - 1,
        Nop(_) => {}
      }
    }

    Ok(acc)
  }

  pub fn fix(&mut self) -> Option<i32> {
    for cur in 0..self.ops.len() {
      let op = &mut self.ops[cur];
      if let Some(flipped) = op.flip() {
        let backup = std::mem::replace(op, flipped);
        if let Ok(acc) = self.run() {
          return Some(acc);
        }
        self.ops[cur] = backup;
      }
    }
    None
  }
}
