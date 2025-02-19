use std::io::{BufRead, BufReader, Write};

use opencv::core::{no_array, Rect};
use opencv::imgproc::{cvt_color_def, ColorConversionCodes};
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

use crate::consts::{
    GRID_CENTRE_SIZE, GRID_LEN, GRID_NUM, GRID_PADDING, GRID_TOPLEFT_X, GRID_TOPLEFT_Y, TEMPL_FILE,
};
use crate::match_bw::BWMatcher;
use crate::span::{Span, Spans};

type Frame = u32;

//      x:0   1  ....
//   y: ┌───┬───┐
//   0  │   │   │
//      ├───┼───┼
//   1  │   │   │
//      ├───┼───┼
//   2  │   │   │
//      ├───┴───┴

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// 見たまま，（画像の中の）長方形領域．
pub struct Sq {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

/// 上の数え方で (x,y) のグリッドの中心部の座標を得る
/// [0, 8]
impl Sq {
    pub fn grid_at(x: i32, y: i32) -> Self {
        Sq {
            x: GRID_TOPLEFT_X + GRID_LEN * x + GRID_PADDING,
            y: GRID_TOPLEFT_Y + GRID_LEN * y + GRID_PADDING,
            w: GRID_CENTRE_SIZE,
            h: GRID_CENTRE_SIZE,
        }
    }
    pub fn into_rect(self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.w,
            height: self.h,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 回答の場所を示す，[-4, 4]
pub struct GridLoc {
    x: i8,
    y: i8,
}

impl GridLoc {
    /// [0, 8] から [-4, 4] へ
    fn from_coordinate(i: u8, j: u8) -> Self {
        GridLoc {
            x: i as i8 - (GRID_NUM / 2) as i8,
            y: j as i8 - (GRID_NUM / 2) as i8,
        }
    }
}

/// ある課題での回答
/// つまり，一つの課題の中でカチカチ動くので，それをまとめたもの
pub struct TrialResult {
    /// この課題全体の開始フレーム
    pub start_frame: usize,
    /// この課題全体の終了フレーム
    pub end_frame: usize,
    /// 途中で選んだ座標を含めた回答一覧
    /// 最初の (0,0) は含める
    pub res: Vec<Span<GridLoc>>,
}

/// 一回通しでやった回答
pub struct Responses {
    rs: Vec<TrialResult>,
}

impl Responses {
    /// report the result like
    /// i,start,end,dur,x,y
    /// 1,124,399,{dur},0,0
    /// 1,400,989,{dur},3,4
    /// 1,990,1230,{dur},4,4
    /// 2,5000,5120,{dur},0,0
    pub fn report_csv<W: Write>(&self, mut paper: &mut W) {
        writeln!(&mut paper, "i,start,end,dur,x,y").unwrap();
        for (i, trial) in self.rs.iter().enumerate() {
            for res_span in trial.res.iter() {
                let index = i + 1;
                let from = res_span.from;
                let to = res_span.to;
                let dur = to - from;
                let x = res_span.val.x;
                let y = res_span.val.y;
                writeln!(&mut paper, "{index},{from},{to},{dur},{x},{y}").unwrap();
            }
        }
    }
}

pub struct ResGatherer {
    matcher: BWMatcher,
}

/// そのまま読み込んだフレーム (frame) に対して，(x,y) が選択されているか？
fn is_this_selected(frame: &Mat, x: i32, y: i32) -> bool {
    let roi = Mat::roi(frame, Sq::grid_at(x, y).into_rect()).expect("is_this_selected::roi");
    let mut grayscale_roi = Mat::default();
    cvt_color_def(
        &roi,
        &mut grayscale_roi,
        ColorConversionCodes::COLOR_BGR2GRAY as i32,
    )
    .unwrap();
    // grayscale にしてて特に絞ってないので， [255.0,0.0,0.0,0.0] みたいに帰ってくる
    let mean = opencv::core::mean(&grayscale_roi, &no_array()).unwrap().0[0];
    println!("\tmean is {mean}, {x}, {y}");
    mean > 200.0
}

impl ResGatherer {
    pub fn from_file(f: &str) -> opencv::Result<Self> {
        let bwm = BWMatcher::from_file(f)?;
        Ok(ResGatherer { matcher: bwm })
    }

    fn gather_responses(&self, vc: &mut VideoCapture) -> Responses {
        let mut frame = Mat::default();
        let mut i = 0;
        while let Ok(b) = vc.read(&mut frame)
            && b
        {
            if self.matcher.does_frame_match(&frame, &None) {
                println!("frame {i} matches:");
                for x in 0..=GRID_NUM {
                    for y in 0..=GRID_NUM {
                        if is_this_selected(&frame, x as i32, y as i32) {
                            println!("\tSelected: {x}, {y} frame is {i}")
                        }
                    }
                }
            }
            i += 1;
        }
        Responses { rs: vec![] }
    }
}

pub fn do_follow_clicks(vc: &mut VideoCapture) -> () {
    let gatherer = ResGatherer::from_file(TEMPL_FILE).unwrap_or_else(|_| panic!("dff:matcher"));
    gatherer.gather_responses(vc);
}
