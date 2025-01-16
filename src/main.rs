use clap::{Parser, Subcommand};
use ikfm2502timeit::consts;
use ikfm2502timeit::find_frames::do_find_frames;
use ikfm2502timeit::load::load_report;
use ikfm2502timeit::prepare::prepare;
use ikfm2502timeit::Spans;

use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, arg_required_else_help = true)]
struct Cli {
    // the video file to process
    #[arg(short, long)]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 参照用の切り抜きを作っておく
    Prepare {
        #[arg(long)]
        sec: f64,
    },
    Process,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let loaded = load_report(&cli.file);
    if loaded.is_none() {
        return ExitCode::FAILURE;
    }
    let mut vc = loaded.unwrap();
    match &cli.command {
        Commands::Prepare { sec } => {
            prepare(&mut vc, *sec);
        }
        Commands::Process => {
            let frames = do_find_frames(&mut vc);
            let spans = Spans::from_bools(&frames);
            let outname = format!("{}.result.csv", &cli.file);
            let mut f = BufWriter::new(fs::File::create(&outname).unwrap());
            spans.report(&mut f, consts::DEFAULT_FPS, None);
            f.flush().unwrap();
        }
    }
    ExitCode::SUCCESS
}
