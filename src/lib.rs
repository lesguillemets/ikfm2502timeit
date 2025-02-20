#![feature(let_chains)]

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub mod base;
pub mod consts;
pub mod extract;
pub mod find_frames;
pub mod follow_clicks;
pub mod load;
pub mod match_bw;
pub mod prepare;
pub mod span;

use crate::span::{FromLine, Span};

/// 単なる区間
pub struct SimpleSpan(Span<()>);

impl FromLine for SimpleSpan {
    fn from_line(line: &str) -> Self {
        let dat: Vec<&str> = line.split(",").collect();
        SimpleSpan(Span {
            val: (),
            from: dat[1].parse().unwrap(),
            to: dat[2].parse().unwrap(),
        })
    }
}

pub struct SimpleSpans {
    dat: Vec<SimpleSpan>,
}

impl SimpleSpans {
    // TODO: better structure??
    pub fn endframes(&self) -> Vec<usize> {
        self.dat.iter().map(|s| s.0.from).collect()
    }
    pub fn report<W: Write>(&self, mut paper: &mut W, fps: f64, sep: Option<&str>) {
        let sep = sep.unwrap_or(",");
        writeln!(
            &mut paper,
            "i{sep}from{sep}to{sep}from_sec{sep}to_sec{sep}dur_frames{sep}dur_seconds"
        )
        .unwrap();
        for (i, SimpleSpan(line)) in self.dat.iter().enumerate() {
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
            return SimpleSpans { dat: vec![] };
        }
        let mut spans = vec![];
        let mut current = from[0];
        let mut last_index = 0;
        for (i, &b) in from.iter().enumerate() {
            match (current, b) {
                // span の終わり
                (true, false) => {
                    spans.push(SimpleSpan(Span {
                        val: (),
                        from: last_index,
                        to: i - 1,
                    }));
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
                SimpleSpan(Span {
                    val: (),
                    from: last_index,
                    to: from.len(),
                })
            })
        }
        SimpleSpans { dat: spans }
    }

    // FIXME: use Result
    // parse from file
    pub fn from_file(f: &str) -> Option<Self> {
        let reader = BufReader::new(File::open(f).ok()?);
        let spans = reader
            .lines()
            .skip(1) // header
            .filter_map(|l| Some(SimpleSpan::from_line(&l.ok()?)))
            .collect();
        Some(SimpleSpans { dat: spans })
    }
}
