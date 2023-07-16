 rust
use std::marker::PhantomData;

pub enum ScanErrorKind {
    Syntax(&'static str),
}

pub trait ScanFromStr<'a>: Sized {
    type Output;
    fn scan_from(s: &'a str) -> Result<(Self::Output, usize), ScanErrorKind>;
}

pub struct Everything<'a, Output=&'a str>(PhantomData<(&'a (), Output)>);

impl<'a, Output> ScanFromStr<'a> for Everything<'a, Output> where &'a str: Into<Output> {
    type Output = Output;
    fn scan_from(s: &'a str) -> Result<(Self::Output, usize), ScanErrorKind> {
        Ok((s.into(), s.len()))
    }
}

fn main() {}
