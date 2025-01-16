#![feature(let_chains)]

pub mod consts;
pub mod find_frames;
pub mod load;
pub mod prepare;

use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use std::process::ExitCode;

use crate::load::{load_video, LoadVideoError};

pub fn load_report(f: &str) -> Option<VideoCapture> {
    match load_video(f) {
        Ok((mut vc, frame_count)) => {
            eprintln!("ready to process video with {frame_count} frames");
            Some(vc)
        }
        Err(LoadVideoError::OpenCVError(oce)) => {
            eprintln!("Error on opencv: {:?}", oce);
            None
        }
        Err(LoadVideoError::NoFrameError) => {
            eprintln!(
                "error: CAP_PROP_FRAME_COUNT didn't return positive number; maybe not a video"
            );
            None
        }
        Err(LoadVideoError::FileNotFoundError) => {
            eprintln!("file not found");
            None
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
