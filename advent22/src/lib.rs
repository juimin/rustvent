use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramArgs {
    #[arg(short, long)]
    input_file: String,
}

// Read the contents of the given file path and write them to the
// mutaable string buffer.
pub fn get_input_contents() -> String {
    let args = ProgramArgs::parse();

    fs::read_to_string(args.input_file)
        .expect(format!("Should have been able to read file").as_str())
}
