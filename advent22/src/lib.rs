use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProgramArgs {
    #[arg(short, long)]
    input_file: String,
}

// Read the contents of the given file path and write them to the
// mutaable string buffer
pub fn read_file_contents(file_path: String) -> String {
    fs::read_to_string(file_path).expect(format!("Should have been able to read file").as_str())
}
