#![feature(let_chains)]

use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_PROP_FRAME_COUNT};
use std::process::ExitCode;

pub fn load(f: &str) -> ExitCode {
    if let Ok(mut vc) = VideoCapture::from_file(f, 0) {
        if let Ok(frame_count) = vc.get(CAP_PROP_FRAME_COUNT)
            && frame_count > 0.0
        {
            eprintln!("ready to process video with {frame_count} frames");
            let frames = find_the_frames(&mut vc);
            ExitCode::SUCCESS
        } else {
            eprintln!(
                "error: CAP_PROP_FRAME_COUNT didn't return positive number; maybe not a video"
            );
            ExitCode::FAILURE
        }
    } else {
        eprintln!("error reading file: probably file not found");
        ExitCode::FAILURE
    }
}

fn find_the_frames(vc: &mut VideoCapture) -> Vec<(usize, usize)> {
    let mut frame = Mat::default();
    while let Ok(b) = vc.read(&mut frame)
        && b
    {
        println!("{frame:?}");
    }
    vec![]
}
