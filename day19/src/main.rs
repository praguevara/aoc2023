use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Condition {
    field: char,    // 'x', 'm', 'a', 's'
    operator: char, // '<', '>'
    value: isize,
}

fn parse_condition(input_str: &str) -> Condition {
    let field = input_str.chars().next().unwrap();
    let operator = input_str.chars().nth(1).unwrap();
    let value = input_str[2..].parse().unwrap();
    Condition {
        field,
        operator,
        value,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rule {
    condition: Option<Condition>,
    then: String,
}

fn parse_rule(input_str: &str) -> Rule {
    let colon_split = input_str.split(':').collect::<Vec<_>>();
    let (condition, then) = if colon_split.len() == 2 {
        (
            Some(parse_condition(colon_split[0])),
            colon_split[1].to_owned(),
        )
    } else {
        (None, colon_split[0].to_owned())
    };
    Rule { condition, then }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_workflow(input_line: &str) -> Workflow {
    let mut parts = input_line.split('{');
    let name = parts.next().unwrap().to_string();
    let rules_str = parts.next().unwrap().split('}').next().unwrap();
    let rules = rules_str.split(',').map(parse_rule).collect();
    Workflow { name, rules }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

fn parse_part(input_line: &str) -> Part {
    let parts = input_line
        .split('{')
        .nth(1)
        .unwrap()
        .split('}')
        .next()
        .unwrap()
        .split(',');

    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    parts.for_each(|part_str| {
        let mut equal_split = part_str.split('=');
        let field = equal_split.next().unwrap();
        let value = equal_split.next().unwrap().parse().unwrap();
        match field {
            "x" => part.x = value,
            "m" => part.m = value,
            "a" => part.a = value,
            "s" => part.s = value,
            _ => panic!("Unknown field: {}", field),
        }
    });

    part
}

fn test_part(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    fn test_part_recursive(
        workflows: &HashMap<String, Workflow>,
        workflow: &Workflow,
        part: &Part,
    ) -> bool {
        for rule in &workflow.rules {
            let mut meets_condition = true;
            if let Some(condition) = &rule.condition {
                let value = match condition.field {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Unknown field: {}", condition.field),
                };
                meets_condition = match condition.operator {
                    '<' => value < condition.value,
                    '>' => value > condition.value,
                    _ => panic!("Unknown operator: {}", condition.operator),
                };
            }

            if meets_condition {
                match rule.then.as_str() {
                    "A" => return true,
                    "R" => return false,
                    name => {
                        return test_part_recursive(workflows, workflows.get(name).unwrap(), part);
                    }
                }
            }
        }
        panic!("No rule met the condition");
    }

    test_part_recursive(workflows, workflows.get("in").unwrap(), part)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct PartRange {
    x: std::ops::RangeInclusive<isize>,
    m: std::ops::RangeInclusive<isize>,
    a: std::ops::RangeInclusive<isize>,
    s: std::ops::RangeInclusive<isize>,
}

impl PartRange {
    fn new() -> Self {
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    fn bisect(self, letter: char, at: isize) -> (Self, Self) {
        let mut right = self.clone();
        let mut left = self;
        match letter {
            'x' => {
                left.x = *left.x.start()..=at - 1;
                right.x = at..=*right.x.end();
            }
            'm' => {
                left.m = *left.m.start()..=at - 1;
                right.m = at..=*right.m.end();
            }
            'a' => {
                left.a = *left.a.start()..=at - 1;
                right.a = at..=*right.a.end();
            }
            's' => {
                left.s = *left.s.start()..=at - 1;
                right.s = at..=*right.s.end();
            }
            _ => panic!("Unknown letter: {}", letter),
        }
        (left, right)
    }

    fn bisect_comparator(self, comparator: char, letter: char, at: isize) -> (Self, Self) {
        match comparator {
            '<' => self.bisect(letter, at),
            '>' => {
                let (l, r) = self.bisect(letter, at + 1);
                (r, l)
            }
            _ => panic!("Unknown comparator: {}", comparator),
        }
    }

    fn cardinality(&self) -> usize {
        self.x.clone().count()
            * self.m.clone().count()
            * self.a.clone().count()
            * self.s.clone().count()
    }
}

#[test]
fn test_ranges() {
    let range = PartRange::new();
    let (matches, doesnt) = range.bisect_comparator('>', 'x', 2000);
    dbg!(&matches);
    dbg!(&doesnt);
    dbg!(matches.cardinality());
    dbg!(doesnt.cardinality());
}

fn part_ranges_cardinality(workflows: &HashMap<String, Workflow>, part_range: PartRange) -> usize {
    fn part_ranges_cardinality_recursive(
        workflows: &HashMap<String, Workflow>,
        workflow: &Workflow,
        mut part_range: PartRange,
    ) -> usize {
        let mut cardinality = 0;
        for rule in &workflow.rules {
            if let Some(ref condition) = rule.condition {
                let (matches, doesnt) = part_range.clone().bisect_comparator(
                    condition.operator,
                    condition.field,
                    condition.value,
                );

                cardinality += match rule.then.as_str() {
                    "A" => matches.cardinality(),
                    "R" => 0,
                    name => part_ranges_cardinality_recursive(
                        workflows,
                        workflows.get(name).unwrap(),
                        matches,
                    ),
                };

                part_range = doesnt;
            } else {
                return cardinality
                    + match rule.then.as_str() {
                        "A" => part_range.cardinality(),
                        "R" => 0,
                        name => part_ranges_cardinality_recursive(
                            workflows,
                            workflows.get(name).unwrap(),
                            part_range,
                        ),
                    };
            }
        }
        cardinality
    }
    part_ranges_cardinality_recursive(workflows, workflows.get("in").unwrap(), part_range)
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut input_split = input_str.split("\r\n\r\n");
    let workflows: HashMap<String, Workflow> = input_split
        .next()
        .unwrap()
        .lines()
        .map(parse_workflow)
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    let parts = input_split
        .next()
        .unwrap()
        .lines()
        .map(parse_part)
        .collect::<Vec<_>>();

    // Part 1
    let part1 = parts
        .iter()
        .filter(|part| test_part(&workflows, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<isize>();
    println!("Part 1: {}", part1);

    // Part 2
    let part2 = part_ranges_cardinality(&workflows, PartRange::new());
    println!("Part 2: {}", part2);
}
