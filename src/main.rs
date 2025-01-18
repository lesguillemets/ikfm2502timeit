use clap::{ArgGroup, Args, Parser, Subcommand};
use ikfm2502timeit::consts;
use ikfm2502timeit::find_frames::do_find_frames;
use ikfm2502timeit::load::load_report;
use ikfm2502timeit::prepare::prepare;
use ikfm2502timeit::Spans;
use opencv::videoio::VideoCapture;

use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, arg_required_else_help = true)]
struct Cli {
    // the video file to process
    #[clap(flatten)]
    file_or_dir: FileOrDir,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct FileOrDir {
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    dir: Option<String>,
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
    let files;
    if let Some(f) = &cli.file_or_dir.file {
        files = vec![f.clone()];
    } else {
        files = fs::read_dir(&cli.file_or_dir.dir.as_ref().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
    }
    let mut loaded: Vec<VideoCapture> = files.iter().filter_map(|f| load_report(f)).collect();
    if loaded.len() == 0 {
        return ExitCode::FAILURE;
    }
    for mut vc in &mut loaded {
        match &cli.command {
            Commands::Prepare { sec } => {
                prepare(&mut vc, *sec);
            }
            Commands::Process => {
                let frames = do_find_frames(&mut vc);
                let spans = Spans::from_bools(&frames);
                let outname = format!("{}.result.csv", &cli.file_or_dir.file.clone().unwrap());
                let mut f = BufWriter::new(fs::File::create(&outname).unwrap());
                spans.report(&mut f, consts::DEFAULT_FPS, None);
                f.flush().unwrap();
            }
        }
    }
    ExitCode::SUCCESS
}
