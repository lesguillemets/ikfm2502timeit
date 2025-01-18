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
    let files: Vec<String>;
    if let Some(f) = &cli.file_or_dir.file {
        files = vec![f.clone()];
    } else {
        let dir_name: &str = cli.file_or_dir.dir.as_deref().unwrap();
        files = fs::read_dir(dir_name)
            .unwrap()
            .map(|e| dir_name.to_string() + &e.unwrap().file_name().into_string().unwrap())
            .collect();
    }
    println!("{files:?}");
    let mut loaded: Vec<(VideoCapture, String)> = files
        .iter()
        .zip(files.iter().cloned())
        .filter_map(|(f, name)| Some((load_report(f)?, name)))
        .collect();
    if loaded.is_empty() {
        return ExitCode::FAILURE;
    }
    for (vc, file_name) in &mut loaded {
        match &cli.command {
            Commands::Prepare { sec } => {
                prepare(vc, *sec);
            }
            Commands::Process => {
                let frames = do_find_frames(vc);
                let spans = Spans::from_bools(&frames);
                let outname = format!("{}.result.csv", &file_name);
                let mut f = BufWriter::new(fs::File::create(&outname).unwrap());
                spans.report(&mut f, consts::DEFAULT_FPS, None);
                f.flush().unwrap();
            }
        }
    }
    ExitCode::SUCCESS
}
