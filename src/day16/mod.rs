use self::rule::*;
use self::ticket::*;

mod rule;
mod ticket;

pub fn run(input: &str) -> (usize, usize) {
  let (rules, mine, others) = parse(input).unwrap();

  let (invalid, valid) = others.into_iter()
    .partition::<Vec<_>, _>(|t| t.is_invalid(&rules));

  let rule_to_field_map = rule_to_field_map(valid, &rules);

  let part1 = invalid.iter()
    .flat_map(|t| t.invalid_values(&rules))
    .sum::<u32>() as usize;

  let part2 = rules.into_iter().enumerate()
    .filter(|(_, rule)| rule.name().starts_with("departure"))
    .map(|(i, _)| mine.field(rule_to_field_map[i]) as usize)
    .product::<usize>();

  (part1, part2)
}

fn parse(input: &str) -> Option<(Vec<Rule<'_>>, Ticket, Vec<Ticket>)> {
  let mut blocks = input.split("\n\n").map(str::lines);

  let rules = blocks.next()?
    .map(Rule::parse)
    .collect::<Option<_>>()?;

  let mine = blocks.next()?
    .nth(1)
    .and_then(Ticket::parse)?;

  let others = blocks.next()?
    .skip(1)
    .map(Ticket::parse)
    .collect::<Option<_>>()?;

  Some((rules, mine, others))
}

fn rule_to_field_map(tickets: Vec<Ticket>, rules: &[Rule<'_>]) -> [usize; FIELDS] {
  let fields = transpose(tickets).unwrap();

  let mut nth_field_matches_n_rules = [0; FIELDS];
  let mut nth_rule_matches_n_fields = [0; FIELDS];
  let mut rule_to_field_map = [0; FIELDS];

  for (r, rule) in rules.iter().enumerate() {
    for (f, field) in fields.iter().enumerate() {
      if field.iter().all(|&v| rule.matches(v)) {
        nth_field_matches_n_rules[f] += 1;
        nth_rule_matches_n_fields[r] += 1;
      }
    }
  }

  for (map, &n) in rule_to_field_map.iter_mut().zip(&nth_rule_matches_n_fields) {
    let correlated = 1 + FIELDS - n;
    *map = nth_field_matches_n_rules.iter()
      .position(|&n| n == correlated)
      .unwrap();
  }

  rule_to_field_map
}

fn transpose<T, M, V>(matrix: M) -> Option<Vec<Vec<T>>>
where
  M: IntoIterator<Item = V>,
  V: IntoIterator<Item = T>
{
  let mut iters = matrix.into_iter()
    .map(IntoIterator::into_iter)
    .collect::<Vec<_>>();

  (0..iters.first()?.size_hint().0)
    .map(|_| iters.iter_mut().map(Iterator::next).collect())
    .collect()
}
