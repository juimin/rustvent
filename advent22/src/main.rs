use clap::Parser;

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
}
