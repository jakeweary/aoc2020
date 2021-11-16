use self::rule::*;
use self::ticket::*;

mod rule;
mod ticket;

pub fn run(input: &str) -> (usize, usize) {
  let mut blocks = input
    .split("\n\n")
    .map(str::lines);

  let rules = blocks.next().unwrap()
    .map(Rule::parse)
    .collect::<Option<Vec<_>>>()
    .unwrap();

  let my_ticket = blocks.next().unwrap()
    .nth(1)
    .map(Ticket::parse)
    .unwrap();

  let nearby_tickets = blocks.next().unwrap()
    .skip(1)
    .map(Ticket::parse)
    .collect::<Option<Vec<_>>>()
    .unwrap();

  let error_rate = nearby_tickets.iter()
    .flat_map(|t| t.invalid_values(&rules))
    .sum::<u32>() as usize;

  (error_rate, 0)
}
