use clap::{Parser, Subcommand};
use ikfm2502timeit::do_load;
use ikfm2502timeit::load::load_video;
use ikfm2502timeit::prepare::prepare;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, arg_required_else_help = true)]
struct Cli {
    // the video file to process
    #[arg(short, long)]
    file: String,

    #[command(subcommand)]
    command: Option<Commands>,
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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Prepare { sec }) => {
            if let Ok(mut cv) = load_video(&cli.file) {
                prepare(&mut cv.0, *sec);
            }
        }
        Some(Commands::Process) => {
            do_load(&cli.file);
        }
        None => {}
    }
}
