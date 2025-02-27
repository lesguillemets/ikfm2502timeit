use crate::consts;

use opencv::core::{sum_elems, CmpTypes, Rect};
use opencv::imgcodecs::{imread, ImreadModes};
use opencv::imgproc::{cvt_color_def, threshold, ColorConversionCodes, ThresholdTypes};
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

#[derive(Debug)]
pub enum FindFramesError {
    TemplateNotFound,
    OpenCVError(opencv::error::Error),
}

pub struct BWMatcher {
    tmpl: Mat,
}

const BW_THRESHOLD: f64 = 32.0;
impl BWMatcher {
    fn new(tmpl: Mat) -> Self {
        BWMatcher { tmpl }
    }

    pub fn from_file(f: &str) -> opencv::Result<Self> {
        let grayscale = imread(f, ImreadModes::IMREAD_GRAYSCALE as i32)?;
        let mut bw = Mat::default();
        threshold(
            &grayscale,
            &mut bw,
            BW_THRESHOLD,
            255.0,
            ThresholdTypes::THRESH_BINARY as i32,
        )?;
        Ok(BWMatcher::new(bw))
    }

    fn check_frame_match(&self, frame: &Mat) -> opencv::Result<f64> {
        let roi = Mat::roi(
            frame,
            Rect {
                x: consts::VA_ROI_X,
                y: consts::VA_ROI_Y,
                width: consts::VA_ROI_W,
                height: consts::VA_ROI_H,
            },
        )?;
        let mut gs_roi = Mat::default();
        cvt_color_def(
            &roi,
            &mut gs_roi,
            ColorConversionCodes::COLOR_BGR2GRAY as i32,
        )?;
        // frame の roi は結局2値化されてここに入る
        let mut bw_roi = Mat::default();
        threshold(
            &gs_roi,
            &mut bw_roi,
            BW_THRESHOLD,
            255.0,
            ThresholdTypes::THRESH_BINARY as i32,
        )?;
        // 比べて違うところを数える
        let mut compared = Mat::default();
        opencv::core::compare(&self.tmpl, &bw_roi, &mut compared, CmpTypes::CMP_NE as i32)?;
        sum_elems(&compared).map(|res| res.0[0])
    }

    pub fn does_frame_match(&self, frame: &Mat, threshold: &Option<f64>) -> bool {
        let score = self
            .check_frame_match(frame)
            .expect("does_frame_match.unwrap");
        score < threshold.unwrap_or(consts::MATCH_BW_THRESHOLD)
    }

    /// true if that frame matches
    fn check_video(&self, vc: &mut VideoCapture, threshold: &Option<f64>) -> Vec<bool> {
        let mut isvas = vec![];
        let mut frame = Mat::default();
        while let Ok(b) = vc.read(&mut frame)
            && b
        {
            isvas.push(self.does_frame_match(&frame, threshold));
            frame = Mat::default();
        }
        isvas
    }
}
pub fn do_find_frames(vc: &mut VideoCapture, threshold: &Option<f64>) -> Vec<bool> {
    let matcher =
        BWMatcher::from_file(consts::TEMPL_FILE).unwrap_or_else(|_| panic!("dff:matcher"));
    matcher.check_video(vc, threshold)
}
