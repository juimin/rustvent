use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: i16,

    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "Running advent22 - day {} on input {}",
        args.day, args.input_file
    );
}
