rust
#![feature(test)]
extern crate test;
use std::str::Chars;

pub struct CharIndices<'a> {
    front_offset: usize,
    iter: Chars<'a>,
}

pub fn before(self_: &mut CharIndices) -> Option<(usize, char)> {
    let pre_len = self_.iter.as_str().len();
    match self_.iter.next() {
        None => None,
        Some(ch) => {
            let index = self_.front_offset;                
            let len = self_.iter.as_str().len();
            self_.front_offset += pre_len - len;
            Some((index, ch))
        }
    }
}

pub fn after(self_: &mut CharIndices) -> Option<(usize, char)> {
    let ch = self_.iter.next()?;
    let index = self_.front_offset;
    self_.front_offset += ch.len_utf8();
    Some((index, ch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn before(b: &mut Bencher) {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let len = s.chars().count();

        b.iter(|| {
            let mut chars = CharIndices { front_offset: 0, iter: s.chars() };
            let mut i = 0;

            while let Some(_) = super::before(&mut chars) {
                i += 1;
            }
            assert_eq!(i, len);
        });
    }

    #[bench]
    fn after(b: &mut Bencher) {
        let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
        let len = s.chars().count();

        b.iter(|| {
            let mut chars = CharIndices { front_offset: 0, iter: s.chars() };
            let mut i = 0;

            while let Some(_) = super::after(&mut chars) {
                i += 1;
            }
            assert_eq!(i, len);
        });
    }
}
