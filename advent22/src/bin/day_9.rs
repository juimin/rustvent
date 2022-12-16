use advent22::get_input_contents;
use std::collections::HashSet;

fn get_next_head_position(head: &(i32, i32), direction: &str) -> (i32, i32) {
    return match direction {
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        _ => panic!("This is invalid blah"),
    };
}

fn get_next_position(&current_knot: &(i32, i32), &leading_knot: &(i32, i32)) -> (i32, i32) {
    // Assumes that the thing is not in range
    let diff = (
        leading_knot.0 - current_knot.0,
        leading_knot.1 - current_knot.1,
    );

    return match diff {
        // Straight
        (0, 2) => (current_knot.0, current_knot.1 + 1),
        (0, -2) => (current_knot.0, current_knot.1 - 1),
        (2, 0) => (current_knot.0 + 1, current_knot.1),
        (-2, 0) => (current_knot.0 - 1, current_knot.1),
        // Diagonal right up
        (1, 2) => (current_knot.0 + 1, current_knot.1 + 1),
        (2, 1) => (current_knot.0 + 1, current_knot.1 + 1),
        (2, 2) => (current_knot.0 + 1, current_knot.1 + 1),
        // Diagonal left up
        (-1, 2) => (current_knot.0 - 1, current_knot.1 + 1),
        (-2, 1) => (current_knot.0 - 1, current_knot.1 + 1),
        (-2, 2) => (current_knot.0 - 1, current_knot.1 + 1),
        // Diagonal right down
        (1, -2) => (current_knot.0 + 1, current_knot.1 - 1),
        (2, -1) => (current_knot.0 + 1, current_knot.1 - 1),
        (2, -2) => (current_knot.0 + 1, current_knot.1 - 1),
        // Diagonal left down
        (-1, -2) => (current_knot.0 - 1, current_knot.1 - 1),
        (-2, -1) => (current_knot.0 - 1, current_knot.1 - 1),
        (-2, -2) => (current_knot.0 - 1, current_knot.1 - 1),
        // same spot
        _ => current_knot.clone(),
    };

    // The leading knot must be diagonal
}

fn knot_should_move(tail_position: &(i32, i32), head_position: &(i32, i32)) -> bool {
    if tail_position == head_position {
        return false;
    }

    let head_in_range = (head_position.0 <= tail_position.0 + 1)
        && (head_position.0 >= tail_position.0 - 1)
        && (head_position.1 <= tail_position.1 + 1)
        && (head_position.1 >= tail_position.1 - 1);

    return !head_in_range;
}

fn get_tail_positions(file_contents: &String, rope_size: usize) {
    let mut rope: Vec<(i32, i32)> = Vec::new();

    for _ in 0..rope_size {
        rope.push((0, 0));
    }

    let mut tail_positions_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_positions_visited.insert((0, 0));

    for line in file_contents.trim().split("\n") {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().expect("there should be a direction");
        let units = tokens
            .next()
            .expect("There should be a thing")
            .parse::<i32>()
            .expect("yes");
        for _ in 0..units {
            for knot_idx in 0..rope.len() {
                if knot_idx == 0 {
                    rope[knot_idx] = get_next_head_position(&rope[0], direction);
                } else {
                    if knot_should_move(&rope[knot_idx], &rope[knot_idx - 1]) {
                        rope[knot_idx] = get_next_position(&rope[knot_idx], &rope[knot_idx - 1]);
                    }
                }
            }
            tail_positions_visited.insert(
                rope.last()
                    .expect("There should be something if rope size is zero")
                    .clone(),
            );
        }
    }
    println!(
        "Unique tail positions visited for rope size {}: {}",
        rope_size,
        tail_positions_visited.len()
    );
}

fn main() {
    let file_contents = get_input_contents();

    get_tail_positions(&file_contents, 2);
    get_tail_positions(&file_contents, 10);
}
