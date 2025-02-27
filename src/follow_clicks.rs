use std::fs;
use std::io::{BufWriter, Write};

use opencv::core::{no_array, Rect};
use opencv::imgproc::{cvt_color_def, ColorConversionCodes};
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

use crate::base::{group_by, Frame};
use crate::consts::{
    GRID_CENTRE_SIZE, GRID_LEN, GRID_NUM, GRID_PADDING, GRID_TOPLEFT_X, GRID_TOPLEFT_Y, TEMPL_FILE,
};
use crate::match_bw::BWMatcher;
use crate::span::Span;

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
            y: (GRID_NUM / 2) as i8 - j as i8,
        }
    }
}

#[derive(Debug)]
/// ある課題での回答
/// つまり，一つの課題の中でカチカチ動くので，それをまとめたもの
pub struct TrialResult {
    /// この課題全体の開始フレーム
    pub start_frame: Frame,
    /// この課題全体の終了フレーム
    pub end_frame: Frame,
    /// 途中で選んだ座標を含めた回答一覧
    /// 最初の (0,0) は含める
    pub res: Vec<Span<GridLoc>>,
}

/// 一回通しでやった回答
pub struct Responses {
    rs: Vec<TrialResult>,
}

impl Responses {
    fn from_indfrval(selections: &[(u32, Frame, GridLoc)]) -> Self {
        if selections.is_empty() {
            return Responses::empty();
        }
        let mut results = vec![];
        let trials = group_by(selections, |p| p.0);
        for trial in trials {
            // この trial のなかでの選択ごとにグループ化
            let selections = group_by(&trial, |p| p.2);
            let trial_result = TrialResult {
                start_frame: trial[0].1,
                end_frame: trial[trial.len() - 1].1,
                res: selections
                    .iter()
                    .map(|gr| Span {
                        val: gr[0].2,
                        from: gr[0].1,
                        to: gr[gr.len() - 1].1,
                    })
                    .collect(),
            };
            results.push(trial_result);
        }
        Responses { rs: results }
    }

    fn empty() -> Self {
        Responses { rs: vec![] }
    }

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
        paper.flush().unwrap();
    }
    /// report the response time to the first click
    /// i,start,end,init_dur,total_dur,first_x,first_y,final_x,final_y,clicks
    /// i: ith trial
    /// start: 評定開始
    /// end: 最初のクリックのフレーム
    /// init_dur: 初動（最初のクリック）までの長さ
    /// total_dur: このtrial全体でどれだけかかったか
    /// first_*: 最初に選んだ点の座標
    /// final_*: 最終的な点の座標
    /// clicks: 何回クリックしたか
    pub fn report_csv_rts<W: Write>(&self, mut paper: &mut W) {
        writeln!(
            &mut paper,
            "i,start,end,init_dur,total_dur,first_x,first_y,final_x,final_y,clicks"
        )
        .unwrap();
        for (i, trial) in self.rs.iter().enumerate() {
            let index = i + 1;
            let start_here = &trial.res[0];
            let from = start_here.from;
            let to = start_here.to;
            let init_dur = start_here.dur();
            let first_choice = if trial.res.len() <= 1 {
                // OK 推す前に録画が終了するケース，あるいは (0,0) をそのまま選ぶケース
                &trial.res[0]
            } else {
                &trial.res[1]
            };
            let first_x = first_choice.val.x;
            let first_y = first_choice.val.y;
            let final_choice = &trial.res[trial.res.len() - 1];
            let total_dur = final_choice.to - start_here.from;
            let final_x = final_choice.val.x;
            let final_y = final_choice.val.y;
            let clicks = trial.res.len() - 1;
            writeln!(&mut paper, "{index},{from},{to},{init_dur},{total_dur},{first_x},{first_y},{final_x},{final_y},{clicks}").unwrap();
        }
        paper.flush().unwrap();
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
    mean > 200.0
}

impl ResGatherer {
    pub fn from_file(f: &str) -> opencv::Result<Self> {
        let bwm = BWMatcher::from_file(f)?;
        Ok(ResGatherer { matcher: bwm })
    }

    fn gather_responses(&self, vc: &mut VideoCapture) -> Responses {
        let mut frame = Mat::default();
        let mut frame_number = 0;
        let mut index: u32 = 0;
        let mut is_start_of_trial = true;
        // 何度目のtrial か，フレーム，そこで選択されたマス
        let mut selections: Vec<(u32, Frame, GridLoc)> = vec![];
        while let Ok(b) = vc.read(&mut frame)
            && b
        {
            // 評定画面についてはチェックする
            if self.matcher.does_frame_match(&frame, &None) {
                if is_start_of_trial {
                    // ここが trial のはじめなのでそれを記録しておく
                    index += 1;
                    is_start_of_trial = false;
                }
                // TODO: here it can be made 100x faster
                let mut selected: Vec<GridLoc> = vec![];
                for x in 0..=GRID_NUM {
                    for y in 0..=GRID_NUM {
                        // x,y が選択されてるか
                        // 選択したフレームだけ全部真っ白になる
                        if is_this_selected(&frame, x as i32, y as i32) {
                            selected.push(GridLoc::from_coordinate(x, y));
                        }
                    }
                }
                if selected.len() == 1 {
                    // 1マスだけ選択されていて平和
                    selections.push((index, frame_number, selected[0]));
                } else {
                    // 全体が光る，OK 押下直後のはず
                    assert_eq!(selected.len(), ((GRID_NUM + 1) * (GRID_NUM + 1)).into());
                    // そうっぽいので，前回選ばれたマスをそのまま使う．
                    let last_selection = selections[selections.len() - 1];
                    assert_eq!(last_selection.1 + 1, frame_number); // ちゃんと直前があるよね？
                    selections.push((index, frame_number, last_selection.2));
                }
            } else {
                // ここは評定画面外．次に評定画面が出てきたら，それはその開始フレームだ
                is_start_of_trial = true;
            }
            // counting the frame manually
            frame_number += 1;
        }
        Responses::from_indfrval(&selections)
    }
}

pub fn do_follow_clicks(vc: &mut VideoCapture, file_name: &str) {
    let gatherer = ResGatherer::from_file(TEMPL_FILE).unwrap_or_else(|_| panic!("dff:matcher"));
    let res = gatherer.gather_responses(vc);
    let outfile_clicks = format!("{}.clicks.csv", &file_name);
    let mut f = BufWriter::new(fs::File::create(&outfile_clicks).unwrap());
    res.report_csv(&mut f);
    f.flush().unwrap();
    let outfile_rts = format!("{}.reactiontimes.csv", &file_name);
    let mut f = BufWriter::new(fs::File::create(&outfile_rts).unwrap());
    res.report_csv_rts(&mut f);
    f.flush().unwrap();
}
