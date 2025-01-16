#![feature(let_chains)]
use opencv::core::{_InputOutputArray, no_array, Rect};
use opencv::imgcodecs::{imread, ImreadModes};
use opencv::imgproc::{match_shapes, ShapeMatchModes};
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

use crate::consts;
use crate::load::load_report;

#[derive(Debug)]
pub enum FindFramesError {
    TemplateNotFound,
    OpenCVError(opencv::error::Error),
}

pub struct Matcher {
    tmpl: Mat,
    match_method: ShapeMatchModes,
    mask: _InputOutputArray,
}

impl Matcher {
    fn new(tmpl: Mat) -> Matcher {
        Matcher {
            tmpl,
            match_method: ShapeMatchModes::CONTOURS_MATCH_I2,
            mask: no_array(),
        }
    }

    fn from_file(f: &str) -> opencv::Result<Self> {
        let template = imread(f, ImreadModes::IMREAD_UNCHANGED as i32)?;
        Ok(Matcher::new(template))
    }

    fn check_frame(&self, frame: &Mat) -> opencv::Result<f64> {
        let roi = Mat::roi(
            frame,
            Rect {
                x: consts::VA_ROI_X,
                y: consts::VA_ROI_Y,
                width: consts::VA_ROI_W,
                height: consts::VA_ROI_H,
            },
        )?;
        match_shapes(&self.tmpl, &roi, self.match_method as i32, 0.0)
    }

    fn check_video(&self, vc: &mut VideoCapture) -> Vec<f64> {
        let mut scores = vec![];
        let mut frame = Mat::default();
        while let Ok(b) = vc.read(&mut frame)
            && b
        {
            scores.push(self.check_frame(&frame).unwrap());
        }
        scores
    }
}

pub fn do_find_frames(vc: &mut VideoCapture) -> Vec<f64> {
    let matcher = Matcher::from_file(consts::TEMPL_FILE).unwrap_or_else(|_| panic!("dff:matcher"));
    matcher.check_video(vc)
}
