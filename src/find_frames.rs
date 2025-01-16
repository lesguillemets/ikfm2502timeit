use opencv::core::{_InputOutputArray, no_array, Rect};
use opencv::imgproc::{match_shapes, ShapeMatchModes};
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

use crate::consts;

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

    fn from_file() {}

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
}
