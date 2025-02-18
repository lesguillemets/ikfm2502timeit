use crate::consts::{GRID_CENTRE_SIZE, GRID_PADDING, GRID_SIZE, GRID_TOPLEFT_X, GRID_TOPLEFT_Y};
use crate::span::{Span, Spans};

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
pub fn grid_at(x: i32, y: i32) -> Sq {
    Sq {
        x: GRID_TOPLEFT_X + GRID_SIZE * x + GRID_PADDING,
        y: GRID_TOPLEFT_Y + GRID_SIZE * y + GRID_PADDING,
        w: GRID_CENTRE_SIZE,
        h: GRID_CENTRE_SIZE,
    }
}

type Frame = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridLoc {
    x: i8,
    y: i8,
}

impl GridLoc {
    fn from_coordinate(i: u8, j: u8) -> Self {
        GridLoc {
            x: i as i8 - 4,
            y: j as i8 - 4,
        }
    }
}

/// ある課題での回答
pub struct Response {
    /// この課題全体の開始フレーム
    pub start_frame: Frame,
    /// この課題全体の終了フレーム
    pub end_frame: Frame,
    /// 途中で選んだ座標を含めた回答一覧
    /// 最初の (0,0) は含める
    pub res: Spans<GridLoc>,
}
