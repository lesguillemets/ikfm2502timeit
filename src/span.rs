use std::fmt::Debug;

pub trait FromLine {
    fn from_line(line: &str) -> Self;
}

#[derive(Debug)]
/// 期間 (from, to) と，その間の値
pub struct Span<T: Debug + Clone> {
    pub val: T,
    pub from: usize,
    pub to: usize,
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
    pub fn endframes(&self) -> Vec<usize> {
        self.dat.iter().map(|s| s.from).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.dat.is_empty()
    }
    pub fn len(&self) -> usize {
        self.dat.len()
    }
}
