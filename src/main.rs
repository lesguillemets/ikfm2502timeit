use clap::Parser;
use ikfm2502timeit::load::load_video;
use ikfm2502timeit::prepare::prepare;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    // the video file to process
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    if let Ok(mut cv) = load_video(&args.file) {
        prepare(&mut cv.0, 18.0);
    }
}
