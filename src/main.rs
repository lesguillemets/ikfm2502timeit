use clap::{Args, Parser, Subcommand};
use glob::glob;
use ikfm2502timeit::consts;
use ikfm2502timeit::extract::get_nth_frames;
use ikfm2502timeit::follow_clicks::do_follow_clicks;
use ikfm2502timeit::load::load_report;
use ikfm2502timeit::match_bw;
use ikfm2502timeit::prepare::prepare;
use ikfm2502timeit::SimpleSpans;
use opencv::core::Vector;
use opencv::imgcodecs::imwrite;

use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
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

    ExtractTrials {
        #[arg(long)]
        frames_before: usize,
    },

    Gather,
}

fn to_bw_filename(file_name: &str) -> String {
    format!("{}.bw.result.csv", &file_name)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    // 扱うべき動画ファイルのリスト
    let files: Vec<String>;
    if let Some(f) = &cli.file_or_dir.file {
        files = vec![f.clone()];
    } else {
        let dir_name: &str = cli.file_or_dir.dir.as_deref().unwrap();
        // `.mov` は仮定する．
        files = glob(&format!("{dir_name}/*.mov"))
            .unwrap()
            .map(|e| {
                dir_name.to_string() + e.unwrap().as_path().file_name().unwrap().to_str().unwrap()
            })
            .collect();
    }
    eprintln!("{files:?}");
    for (mut vc, file_name) in files
        .iter()
        .zip(files.iter().cloned())
        // ここで video 以外ははじけてると思うんだけど
        .filter_map(|(f, name)| Some((load_report(f)?, name)))
    {
        match &cli.command {
            Commands::Prepare { sec } => {
                prepare(&mut vc, *sec);
            }
            Commands::Process => {
                let frames = match_bw::do_find_frames(&mut vc, &None);
                let spans = SimpleSpans::from_bools(&frames);
                let outname = to_bw_filename(&file_name);
                let mut f = BufWriter::new(fs::File::create(&outname).unwrap());
                spans.report(&mut f, consts::DEFAULT_FPS, None);
                f.flush().unwrap();
            }
            Commands::ExtractTrials { frames_before } => {
                let the_file = Path::new(&file_name);
                let base_name: &str = the_file.file_stem().unwrap().to_str().unwrap();
                // .mov を落としてディレクトリの名前とする
                let out_dir = Path::new(&file_name[..file_name.len() - 4]);
                if out_dir.is_file() {
                    panic!("a FILE named {out_dir:?} exists!!");
                }
                if !out_dir.exists() {
                    fs::create_dir(out_dir).unwrap();
                }
                // ここにフレームを書き込むようにするわけですね．
                let parsed = SimpleSpans::from_file(&to_bw_filename(&file_name)).unwrap();
                let frames: Vec<usize> = parsed
                    .endframes()
                    .iter()
                    .map(|frame| frame - frames_before)
                    .collect();
                for (frame, img) in get_nth_frames(&mut vc, &frames).unwrap() {
                    let outfile = out_dir.join(format!("{base_name}_{frame:05}.jpg"));
                    eprintln!("writing {outfile:?}");
                    imwrite(outfile.to_str().unwrap(), &img, &Vector::new()).unwrap();
                    eprintln!("done: writing {outfile:?}");
                }
            }
            Commands::Gather => {
                do_follow_clicks(&mut vc, &file_name);
            }
        }
    }
    ExitCode::SUCCESS
}
