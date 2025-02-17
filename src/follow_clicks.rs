use crate::consts::{GRID_CENTRE_SIZE, GRID_PADDING, GRID_SIZE, GRID_TOPLEFT_X, GRID_TOPLEFT_Y};

//      x:0   1  ....
//   y: ┌───┬───┐
//   0  │   │   │
//      ├───┼───┼
//   1  │   │   │
//      ├───┼───┼
//   2  │   │   │
//      ├───┴───┴

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// 見たまま，長方形領域．
pub struct Sq {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

/// 上の数え方で (x,y) のグリッドの中心部の座標
pub fn grid_at(x: i32, y: i32) -> Sq {
    Sq {
        x: GRID_TOPLEFT_X + GRID_SIZE * x + GRID_PADDING,
        y: GRID_TOPLEFT_Y + GRID_SIZE * y + GRID_PADDING,
        w: GRID_CENTRE_SIZE,
        h: GRID_CENTRE_SIZE,
    }
}
