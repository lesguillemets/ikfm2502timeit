#![feature(let_chains)]

pub mod consts;
pub mod find_frames;
pub mod load;
pub mod prepare;

use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use std::process::ExitCode;

fn find_the_frames(vc: &mut VideoCapture) -> Vec<(usize, usize)> {
    let mut frame = Mat::default();
    // while let Ok(b) = vc.read(&mut frame)
    //     && b
    // {
    //     println!("{frame:?}");
    // }
    vec![]
}
