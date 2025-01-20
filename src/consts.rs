/// ROI としてとる場所
pub const VA_ROI_X: i32 = 2550;
pub const VA_ROI_Y: i32 = 790;
pub const VA_ROI_W: i32 = 90;
pub const VA_ROI_H: i32 = 50;

/// 切り出したROIを保存する先
pub const TEMPL_FILE: &str = "./data/va_roi.png";

/// match_shapes(&tmpl, &roi, *,*) の閾値
pub const MATCH_SHAPES_THRESHOLD: f64 = 0.005;

/// 2値化して，見比べて，異なるピクセルがこれ以下ならOK
pub const MATCH_BW_THRESHOLD: f64 = 50.0;

/// デフォルトのFPS. 本来はファイルが持ってる情報だが，
/// 取り直すのが面倒なこともあるので…
pub const DEFAULT_FPS: f64 = 30.0;
