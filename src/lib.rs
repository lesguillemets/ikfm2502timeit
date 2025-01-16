#![feature(let_chains)]

mod load;

use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use std::process::ExitCode;

use crate::load::{load_video, LoadVideoError};

pub fn do_load(f: &str) -> ExitCode {
    match load_video(f) {
        Ok((mut vc, frame_count)) => {
            eprintln!("ready to process video with {frame_count} frames");
            let frames = find_the_frames(&mut vc);
            ExitCode::SUCCESS
        }
        Err(LoadVideoError::OpenCVError(oce)) => {
            eprintln!("Error on opencv: {:?}", oce);
            ExitCode::FAILURE
        }
        Err(LoadVideoError::NoFrameError) => {
            eprintln!(
                "error: CAP_PROP_FRAME_COUNT didn't return positive number; maybe not a video"
            );
            ExitCode::FAILURE
        }
        Err(LoadVideoError::FileNotFoundError) => {
            eprintln!("file not found");
            ExitCode::FAILURE
        }
    }
}

fn find_the_frames(vc: &mut VideoCapture) -> Vec<(usize, usize)> {
    let mut frame = Mat::default();
    // while let Ok(b) = vc.read(&mut frame)
    //     && b
    // {
    //     println!("{frame:?}");
    // }
    vec![]
}
