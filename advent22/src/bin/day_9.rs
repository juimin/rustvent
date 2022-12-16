use advent22::get_input_contents;
use std::collections::HashSet;

fn apply_move(mut head_position: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "U" => head_position.1 += 1,
        "D" => head_position.1 -= 1,
        "L" => head_position.0 -= 1,
        "R" => head_position.0 += 1,
        _ => panic!("THis is invalid blah"),
    }
    return head_position;
}

fn tail_should_move(tail_position: &(i32, i32), head_position: &(i32, i32)) -> bool {
    if tail_position == head_position {
        return false;
    }

    return (head_position.0 <= tail_position.0 + 1)
        && (head_position.0 >= tail_position.0 - 1)
        && (head_position.1 <= tail_position.1 + 1)
        && (head_position.1 >= tail_position.1 - 1);
}

fn main() {
    let file_contents = get_input_contents();

    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut tail_positions_visited: HashSet<(i32, i32)> = HashSet::new();

    for line in file_contents.trim().split("\n") {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().expect("there should be a direction");
        let units = tokens
            .next()
            .expect("There should be a thing")
            .parse::<i32>()
            .expect("yes");

        for _ in 0..units {
            let new_head_position = apply_move(head_position, direction);
            if tail_should_move(&tail_position, &new_head_position) {}
        }
    }
}
