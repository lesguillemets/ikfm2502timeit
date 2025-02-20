use std::fmt::Debug;

use crate::base::Frame;

pub trait FromLine {
    fn from_line(line: &str) -> Self;
}

#[derive(Debug)]
/// 期間 (from, to) と，その間の値
pub struct Span<T: Debug + Clone> {
    pub val: T,
    pub from: Frame,
    pub to: Frame,
}

impl<T: Debug + Clone> Span<T> {
    pub fn dur(&self) -> Frame {
        self.to - self.from
    }
}

#[derive(Debug)]
pub struct Spans<T: Debug + Clone> {
    dat: Vec<Span<T>>,
}
// Does this make sense?
// impl AsRef<Span<()>> for SimpleSpan {
//     fn as_ref(&self) -> &Span<()> {
//         &self.0
//     }
// }

impl<T> Spans<T>
where
    T: Debug + Clone,
{
    pub fn endframes(&self) -> Vec<Frame> {
        self.dat.iter().map(|s| s.from).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.dat.is_empty()
    }
    pub fn len(&self) -> usize {
        self.dat.len()
    }
}

impl<T> Spans<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq,
{
    /// (frame, val) のvec を，連続するものをつなげて
    /// Spans にする
    pub fn from_frames(data: &[(Frame, T)]) -> Self {
        Spans { dat: vec![] }
    }
}
