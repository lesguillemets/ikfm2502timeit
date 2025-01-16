#![feature(let_chains)]

use opencv::error;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_PROP_FRAME_COUNT};
use std::path::Path;
use std::process::ExitCode;

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

#[derive(Debug)]
/// OpenCVに起因するエラー，
/// get(CAP_PROP_FRAME_COUNT) が0 （多分動画じゃない），
/// ファイルが見つからない
pub enum LoadVideoError {
    OpenCVError(opencv::error::Error),
    NoFrameError,
    FileNotFoundError,
}

impl From<opencv::error::Error> for LoadVideoError {
    fn from(err: opencv::error::Error) -> LoadVideoError {
        LoadVideoError::OpenCVError(err)
    }
}

/// file is given as a &str. Returns `([opencv::videoio::VideoCapture], usize)`
/// where usize denotes for the frame count for the video.
pub fn load_video(f: &str) -> Result<(VideoCapture, usize), LoadVideoError> {
    if !Path::new(&f).is_file() {
        return Err(LoadVideoError::FileNotFoundError);
    }
    let vc = VideoCapture::from_file(f, 0)?;
    let frame_count = vc.get(CAP_PROP_FRAME_COUNT)?;
    if frame_count > 0.0 {
        Ok((vc, frame_count as usize))
    } else {
        Err(LoadVideoError::NoFrameError)
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
