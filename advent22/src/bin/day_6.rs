use advent22::get_input_contents;
use std::collections::HashSet;

fn detect_start_of_marker(s: &str, start_idx: usize, marker_size: i32) -> i32 {
    let mut idx: usize = start_idx;
    let mut character_set: HashSet<char> = HashSet::new();
    let s_bytes = s.as_bytes();

    while idx < s.len() - marker_size as usize {
        for i in 0..marker_size {
            character_set.insert(s_bytes[idx + i as usize] as char);
        }
        if character_set.len() == marker_size as usize {
            return idx as i32 + marker_size;
        } else {
            character_set.clear();
            idx += 1;
        }
    }

    return -1;
}

fn main() {
    let file_contents = get_input_contents();

    for line in file_contents.trim().split("\n") {
        let start_of_packet = detect_start_of_marker(&line, 0, 4);
        println!("Start of packet: {}", start_of_packet);
        let start_of_message = detect_start_of_marker(&line, 0, 14);
        println!("Start of message: {}", start_of_message);
    }
}
