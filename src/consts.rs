/// ROI としてとる場所
pub const VA_ROI_X: i32 = 2550;
pub const VA_ROI_Y: i32 = 790;
pub const VA_ROI_W: i32 = 90;
pub const VA_ROI_H: i32 = 50;

/// 切り出したROIを保存する先
pub const TEMPL_FILE: &str = "./data/va_roi.png";

/// match_shapes(&tmpl, &roi, *,*) の閾値
pub const MATCH_SHAPES_THRESHOLD: f64 = 0.05;

/// 2値化して，見比べて，異なる分がこれ以下ならOK
/// ピクセルってわけでもなさそうなんだけど 255*pix なのかな…？？
pub const MATCH_BW_THRESHOLD: f64 = 10000.0;

/// デフォルトのFPS. 本来はファイルが持ってる情報だが，
/// 取り直すのが面倒なこともあるので…
pub const DEFAULT_FPS: f64 = 30.0;

/// 評定グリッドの一番左上，白線の交点
pub const GRID_TOPLEFT_X: i32 = 2643;
pub const GRID_TOPLEFT_Y: i32 = 617;

/// 評定の小さいグリッドの大きさ (px)．
/// 気分的には左・上側の辺自体から，右・下側の辺の直前の黒いところまで．
/// 途中で1px ずれるが，GRID_TOPLEFT_Z + 44*Z (z \in {X,Y}) が
/// そのグリッドの左上の座標になる
pub const GRID_LEN: i32 = 44;
/// 何個あるか
pub const GRID_NUM: u8 = 8;

/// 実際に個々のグリッドを見に行く際，上左端から
/// GRID_PADDING だけずらして GRID_CENTRE_SIZE の正方形をとる
pub const GRID_PADDING: i32 = 14;
pub const GRID_CENTRE_SIZE: i32 = 16;
