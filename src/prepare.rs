use opencv::core::{Rect, Vector};
use opencv::imgcodecs::{imwrite, ImwriteFlags};
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_PROP_FPS, CAP_PROP_POS_FRAMES};

use crate::consts;

/// 与えられたファイルの `at_second` 秒目から該当領域を切り出して保存
pub fn prepare(vc: &mut VideoCapture, at_second: f64) {
    let fps = vc.get(CAP_PROP_FPS).unwrap();
    let frame = (fps * at_second).floor();
    vc.set(CAP_PROP_POS_FRAMES, frame).unwrap();

    let mut the_frame = Mat::default();
    if !vc.read(&mut the_frame).unwrap() {
        eprintln!("prepare:error: video not long enough");
    }
    let roi = Mat::roi(
        &the_frame,
        Rect {
            x: consts::VA_ROI_X,
            y: consts::VA_ROI_Y,
            width: consts::VA_ROI_W,
            height: consts::VA_ROI_H,
        },
    )
    .unwrap();
    imwrite(
        "./data/va_roi.png",
        &roi,
        &Vector::from_slice(&[ImwriteFlags::IMWRITE_PNG_COMPRESSION as i32, 9]),
    )
    .unwrap();
}
