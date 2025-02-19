use std::io::{BufRead, BufReader, Write};

use crate::consts::{
    GRID_CENTRE_SIZE, GRID_LEN, GRID_NUM, GRID_PADDING, GRID_TOPLEFT_X, GRID_TOPLEFT_Y,
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
pub fn grid_at(x: i32, y: i32) -> Sq {
    Sq {
        x: GRID_TOPLEFT_X + GRID_LEN * x + GRID_PADDING,
        y: GRID_TOPLEFT_Y + GRID_LEN * y + GRID_PADDING,
        w: GRID_CENTRE_SIZE,
        h: GRID_CENTRE_SIZE,
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
