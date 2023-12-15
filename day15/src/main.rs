fn parse_input(input: &str) -> Vec<&str> {
    input.lines().next().unwrap().split(',').collect()
}

fn hash<'a>(input: impl Into<&'a [u8]>) -> u8 {
    let mut current: u8 = 0;

    for c in input.into() {
        current = current.overflowing_add(*c).0;
        current = current.overflowing_mul(17).0;
    }

    current
}

fn arrange_boxes<'a>(lenses: &Vec<&'a str>) -> [Vec<(&'a str, usize)>; 256] {
    let mut boxes: [Vec<(&'a str, usize)>; 256] = core::array::from_fn(|_| Vec::new());

    for lens in lenses {
        let label_op_idx = lens
            .as_bytes()
            .iter()
            .position(|c| *c == b'-' || *c == b'=')
            .unwrap();
        let split = lens.split_at(label_op_idx);
        let box_idx = hash(split.0.as_bytes());
        let target_box = &mut boxes[box_idx as usize];
        let op = lens.as_bytes()[label_op_idx];
        match op {
            b'=' => {
                let focal_length = split.1[1..].parse::<usize>().unwrap();
                if let Some(existing_idx) =
                    target_box.iter().position(|(label, _)| *label == split.0)
                {
                    target_box[existing_idx].1 = focal_length;
                } else {
                    target_box.push((split.0, focal_length));
                }
            }
            b'-' => {
                if let Some(target_idx) = target_box.iter().position(|(label, _)| *label == split.0)
                {
                    target_box.remove(target_idx);
                }
            }
            e => panic!("invalid label operation, got {}", e as char),
        }
    }

    boxes
}

fn compute_focusing_power(boxes: &[Vec<(&str, usize)>; 256]) -> usize {
    let mut result = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (slot, (_, focal_length)) in b.iter().enumerate() {
            result += (i + 1) * (slot + 1) * focal_length;
        }
    }

    result
}

fn main() {
    let input_str = include_str!("../sample.txt");
    let input = parse_input(input_str);
    let result = input
        .iter()
        .map(|v| hash(v.as_bytes()) as usize)
        .sum::<usize>();

    println!("part 1: {}", result);
    let boxes = arrange_boxes(&input);
    let result = compute_focusing_power(&boxes);
    println!("part 2: {}", result);
}
