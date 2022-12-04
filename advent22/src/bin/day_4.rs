use advent22::get_input_contents;

fn range_to_numbers(line: &str) -> (i32, i32) {
    let mut range_split = line.split("-");
    let first = range_split.next().expect("failed to get first digit");
    let second = range_split.next().expect("failed to get second digit");

    return (
        first
            .parse::<i32>()
            .expect("failed to convert first to int"),
        second
            .parse::<i32>()
            .expect("failed to convert first to int"),
    );
}

fn get_ranges(line: &str) -> (i32, i32, i32, i32) {
    let mut range_split = line.split(",");
    let first = range_split.next().expect("failed to get first range");
    let second = range_split.next().expect("failed to get second range");

    let first_range = range_to_numbers(first);
    let second_range = range_to_numbers(second);

    return (first_range.0, first_range.1, second_range.0, second_range.1);
}

fn check_contains(ranges: (i32, i32, i32, i32)) -> bool {
    if ranges.0 <= ranges.2 && ranges.1 >= ranges.3 {
        return true;
    }
    if ranges.2 <= ranges.0 && ranges.3 >= ranges.1 {
        return true;
    }
    return false;
}

fn check_overlaps(ranges: (i32, i32, i32, i32)) -> bool {
    if ranges.2 >= ranges.0 && ranges.2 <= ranges.1 {
        return true;
    }
    if ranges.3 >= ranges.0 && ranges.3 <= ranges.1 {
        return true;
    }
    if ranges.0 >= ranges.2 && ranges.0 <= ranges.3 {
        return true;
    }
    if ranges.1 >= ranges.2 && ranges.1 <= ranges.3 {
        return true;
    }
    return false;
}

fn main() {
    let mut fully_contained_counter = 0;
    let mut overlap_counter = 0;

    let file_contents: String = get_input_contents();

    for line in file_contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let ranges = get_ranges(line);
        if check_contains(ranges) {
            fully_contained_counter += 1;
        }
        if check_overlaps(ranges) {
            overlap_counter += 1;
        }
    }

    println!("Fully contained pairs: {}", fully_contained_counter);
    println!("Overlapping pairs: {}", overlap_counter);
}
