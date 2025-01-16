use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_PROP_FRAME_COUNT};

pub fn load(f: &str) {
    if let Ok(vc) = VideoCapture::from_file(f, 0) {
        println!(
            "ready to process video with {:?} frames",
            vc.get(CAP_PROP_FRAME_COUNT)
        );
    } else {
        println!("failed to read the video?");
    }
}
