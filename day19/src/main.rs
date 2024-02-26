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
        rule_name: &str,
        part: &Part,
    ) -> bool {
        let workflow = workflows.get(rule_name).unwrap();
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
                        return test_part_recursive(workflows, name, part);
                    }
                }
            }
        }
        panic!("No rule met the condition");
    }

    test_part_recursive(workflows, "in", part)
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
}
