use self::rule::*;
use self::ticket::*;

mod rule;
mod ticket;

pub fn run(input: &str) -> (u32, u64) {
  let (rules, mine, others) = parse(input).unwrap();

  let (invalid, valid) = others.into_iter()
    .partition::<Vec<_>, _>(|t| t.is_invalid(&rules));

  let rule_to_field_map = rule_to_field_map(&rules, valid);

  let part1 = invalid.iter()
    .flat_map(|t| t.invalid_values(&rules))
    .sum();

  let part2 = rules.into_iter().enumerate()
    .filter(|(_, rule)| rule.name().starts_with("departure"))
    .map(|(rule, _)| mine.field(rule_to_field_map[rule]) as u64)
    .product();

  (part1, part2)
}

fn parse(input: &str) -> Option<(Vec<Rule<'_>>, Ticket, Vec<Ticket>)> {
  let mut blocks = input.split("\n\n").map(str::lines);

  let rules = blocks.next()?.map(Rule::parse).collect::<Option<_>>()?;
  let mine = blocks.next()?.nth(1).and_then(Ticket::parse)?;
  let others = blocks.next()?.skip(1).map(Ticket::parse).collect::<Option<_>>()?;

  Some((rules, mine, others))
}

fn rule_to_field_map(rules: &[Rule<'_>], tickets: Vec<Ticket>) -> [usize; FIELDS] {
  let fields = transpose::<_, _, _, Vec<_>>(tickets);

  let mut nth_field_matches_n_rules = [0; FIELDS];
  let mut nth_rule_matches_n_fields = [0; FIELDS];
  let mut map = [0; FIELDS];

  for (n_rules, field) in nth_field_matches_n_rules.iter_mut().zip(fields) {
    for (n_fields, rule) in nth_rule_matches_n_fields.iter_mut().zip(rules) {
      if field.iter().all(|&v| rule.matches(v)) {
        *n_rules += 1;
        *n_fields += 1;
      }
    }
  }

  for (map, &n) in map.iter_mut().zip(&nth_rule_matches_n_fields) {
    let correlated = 1 + FIELDS - n;
    *map = nth_field_matches_n_rules.iter()
      .position(|&n| n == correlated)
      .unwrap();
  }

  map
}

fn transpose<T, V, M, I>(matrix: M) -> impl Iterator<Item = I>
where
  V: IntoIterator<Item = T>,
  M: IntoIterator<Item = V>,
  I: FromIterator<T>
{
  let mut iters = matrix.into_iter()
    .map(IntoIterator::into_iter)
    .collect::<Vec<_>>();

  std::iter::from_fn(move || {
    iters.iter_mut().map(Iterator::next).collect()
  })
}
