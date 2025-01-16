use clap::Parser;

use ikfm2502timeit::load;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    // the video file to process
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
    load(&args.file);
}
