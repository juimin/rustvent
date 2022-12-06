use advent22::get_input_contents;
use std::collections::HashSet;

fn detect_start_of_marker(s: &str, marker_size: usize) -> i32 {
    let mut idx: usize = 0;
    let mut character_set: HashSet<char> = HashSet::new();
    let s_bytes = s.as_bytes();

    while idx < s.len() - marker_size {
        for i in 0..marker_size {
            character_set.insert(s_bytes[idx + i as usize] as char);
        }
        if character_set.len() == marker_size as usize {
            return (idx + marker_size) as i32;
        } else {
            character_set.clear();
            idx += 1;
        }
    }

    return -1;
}

fn main() {
    let file_contents: String = get_input_contents();

    for line in file_contents.trim().split("\n") {
        let start_of_packet: i32 = detect_start_of_marker(&line, 4);
        println!("Start of packet: {}", start_of_packet);
        let start_of_message: i32 = detect_start_of_marker(&line, 14);
        println!("Start of message: {}", start_of_message);
    }
}
