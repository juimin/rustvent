use clap::Parser;

mod io_handling;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: i16,

    #[arg(short, long)]
    input_file: String,

    #[arg[short, long, default_value_t = false]]
    test_file: bool,
}

fn main() {
    let args = Args::parse();

    println!(
        "Running advent22 - day {} on input {}",
        args.day, args.input_file
    );

    if args.test_file {
        let file_contents: String = io_handling::read_file_contents(args.input_file);
        println!("{}", file_contents)
    }
}
