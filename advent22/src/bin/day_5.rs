use advent22::get_input_contents;

fn parse_initial_state(lines: &String) -> Vec<Vec<char>> {
    let lines_split: Vec<&str> = lines.split("\n").collect();
    let stack_count = (lines_split[0].len() + 1) / 4;

    let mut initial_state: Vec<Vec<char>> = Vec::new();

    for _ in 0..stack_count {
        initial_state.push(Vec::new());
    }

    for line in lines_split {
        if line.len() == 0 {
            continue;
        }
        let line_bytes = line.as_bytes();
        for i in 0..stack_count {
            let c = line_bytes[1 + (4 * i)] as char;
            if c.is_alphabetic() {
                initial_state[i].push(c);
            }
        }
    }

    for index in 0..initial_state.len() {
        initial_state[index].reverse();
    }

    return initial_state;
}

fn parse_moves_as_9000(state: &mut Vec<Vec<char>>, input: &String) {
    for line in input.split("\n") {
        if !line.starts_with("move") {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let qnt = tokens[1].parse::<usize>().expect("qnt should be numeric");
        let source = tokens[3]
            .parse::<usize>()
            .expect("source should be numeric")
            - 1;
        let dest = tokens[5].parse::<usize>().expect("dest should be numeric") - 1;

        for _ in 0..qnt {
            if state[source].len() > 0 {
                let c = state[source].pop().expect("something is here");
                state[dest].push(c);
            }
        }
    }
}

fn parse_moves_as_9001(state: &mut Vec<Vec<char>>, input: &String) {
    for line in input.split("\n") {
        if !line.starts_with("move") {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let qnt = tokens[1].parse::<usize>().expect("qnt should be numeric");
        let source = tokens[3]
            .parse::<usize>()
            .expect("source should be numeric")
            - 1;
        let dest = tokens[5].parse::<usize>().expect("dest should be numeric") - 1;

        let mut buffer: Vec<char> = Vec::new();
        for _ in 0..qnt {
            if state[source].len() > 0 {
                let c = state[source].pop().expect("something is here");
                buffer.push(c);
            }
        }
        for _ in 0..buffer.len() {
            let c = buffer.pop().expect("this should exist");
            state[dest].push(c);
        }
    }
}

fn main() {
    let file_contents = get_input_contents();

    let initial_state_string = file_contents
        .split("move")
        .next()
        .expect("expected an initial state before moves")
        .to_string();

    let mut state = parse_initial_state(&initial_state_string);
    let mut second_state = state.clone();

    parse_moves_as_9000(&mut state, &file_contents);
    parse_moves_as_9001(&mut second_state, &file_contents);

    print!("Top of each stack with the CrateMover 9000: ");
    for stack in state {
        if stack.len() > 0 {
            print!("{}", stack.last().expect("something should be there"));
        }
    }
    println!();

    print!("Top of each stack with the CrateMover 9001: ");
    for stack in second_state {
        if stack.len() > 0 {
            print!("{}", stack.last().expect("something should be there"));
        }
    }
}
