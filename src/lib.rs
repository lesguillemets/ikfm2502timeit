#![feature(let_chains)]

use std::io::Write;

pub mod consts;
pub mod find_frames;
pub mod load;
pub mod match_bw;
pub mod prepare;

pub struct Span {
    from: usize,
    to: usize,
}

pub struct Spans {
    dat: Vec<Span>,
}

impl Spans {
    pub fn report<W: Write>(&self, mut paper: &mut W, fps: f64, sep: Option<&str>) {
        let sep = sep.unwrap_or(",");
        writeln!(
            &mut paper,
            "i{sep}from{sep}to{sep}from_sec{sep}to_sec{sep}dur_frames{sep}dur_seconds"
        )
        .unwrap();
        for (i, line) in self.dat.iter().enumerate() {
            let index = i + 1;
            let from = line.from;
            let to = line.to;
            let from_sec = from as f64 / fps;
            let to_sec = to as f64 / fps;
            let dur_frames = to - from;
            let dur_seconds = to_sec - from_sec;
            writeln!(&mut paper,
                "{index}{sep}{from}{sep}{to}{sep}{from_sec}{sep}{to_sec}{sep}{dur_frames}{sep}{dur_seconds}"
                ).unwrap();
        }
    }

    pub fn from_bools(from: &[bool]) -> Self {
        if from.is_empty() {
            return Spans { dat: vec![] };
        }
        let mut spans = vec![];
        let mut current = from[0];
        let mut last_index = 0;
        for (i, &b) in from.iter().enumerate() {
            match (current, b) {
                // span の終わり
                (true, false) => {
                    spans.push(Span {
                        from: last_index,
                        to: i - 1,
                    });
                }
                // span のはじまり}
                (false, true) => {
                    last_index = i;
                }
                // 関係ないところ
                (false, false) => (),
                // span の途中
                (true, true) => (),
            }
            current = b;
        }
        if current {
            spans.push({
                Span {
                    from: last_index,
                    to: from.len(),
                }
            })
        }
        Spans { dat: spans }
    }
}
