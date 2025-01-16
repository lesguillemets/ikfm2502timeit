use clap::Parser;
use ikfm2502timeit::load;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    // the video file to process
    #[arg(short, long)]
    file: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    println!("{args:?}");
    load(&args.file)
}
