pub mod one;

pub fn run(day: u8, filename: &String) {
    match day {
        1 => one::sonar_sweep(filename),
        _ => println!("Not Implemented\n"),
    }
}
