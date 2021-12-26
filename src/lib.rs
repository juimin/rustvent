use std::fs;

// Base Reader
pub fn read_file_with_sep(file: &String, sep: &str) -> Vec<String> {
    // Read the contents from the given file
    let contents = fs::read_to_string(file).expect("Error reading file");
    let contents = contents.trim();
    // Split the values, convert to string, filter empty strings, and collect them
    return contents.split(sep).map(|s| s.to_string()).collect();
}

pub fn file_to_int_vector(file: &String, sep: &str) -> Vec<i64> {
    let values = read_file_with_sep(file, sep);
    return values.iter().map(|s| s.parse::<i64>().unwrap()).collect();
}
