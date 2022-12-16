use advent22::get_input_contents;
use std::collections::HashSet;

fn apply_move(head: &(i32, i32), direction: &str) -> (i32, i32) {
    return match direction {
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        _ => panic!("THis is invalid blah"),
    };
}

fn tail_should_move(tail_position: &(i32, i32), head_position: &(i32, i32)) -> bool {
    if tail_position == head_position {
        return false;
    }

    let head_in_range = (head_position.0 <= tail_position.0 + 1)
        && (head_position.0 >= tail_position.0 - 1)
        && (head_position.1 <= tail_position.1 + 1)
        && (head_position.1 >= tail_position.1 - 1);

    return !head_in_range;
}

fn main() {
    let file_contents = get_input_contents();

    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut tail_positions_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_positions_visited.insert(tail_position);

    for line in file_contents.trim().split("\n") {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().expect("there should be a direction");
        let units = tokens
            .next()
            .expect("There should be a thing")
            .parse::<i32>()
            .expect("yes");
        for _ in 0..units {
            println!("H: {:?} T: {:?}", head_position, tail_position);
            let new_head_position = apply_move(&head_position, direction);
            if tail_should_move(&tail_position, &new_head_position) {
                tail_position = head_position.clone();
                tail_positions_visited.insert(tail_position);
            }
            head_position = new_head_position;
        }
    }

    println!(
        "Unique tail positions visited: {}",
        tail_positions_visited.len()
    )
}
