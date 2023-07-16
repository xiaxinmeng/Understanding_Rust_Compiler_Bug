rust
#![feature(test, box_syntax)]

extern crate test;

use std::io::{Write, Error, ErrorKind};

const DATA_SIZE: usize = 200;

#[derive(Copy, Clone)]
struct Huge {
    data: [u8; DATA_SIZE],
}

struct HugeIter<'a> {
    cur_val: Huge,
    buf: &'a [u8],
}

impl<'a> HugeIter<'a> {
    fn next(&mut self) -> Option<&Huge> {
        if let Ok(()) = parse_huge(&mut self.buf, &mut self.cur_val) {
            Some(&self.cur_val)
        } else {
            None
        }
    }
    
    fn next_roundtrip_caller(&mut self) -> Option<&Huge> {
        unsafe {
            let mut val = Huge { data: ::std::mem::uninitialized() };
            let result = match parse_huge(&mut self.buf, &mut val) {
                Ok(()) => Ok(val),
                Err(e) => Err(e)
            };
            if let Ok(ref val) = result {
                self.cur_val = *val;
                Some(&self.cur_val)
            } else {
                None
            }
        }
    }
}

struct HugeResultIter<'a> {
    cur_val: Result<Huge, Error>,
    buf: &'a [u8],
}

impl<'a> HugeResultIter<'a> {
    fn next(&mut self) -> Option<&Huge> {
        parse_huge_result(&mut self.buf, &mut self.cur_val);
        self.cur_val.as_ref().ok()
    }
    fn next_copy(&mut self) -> Option<&Huge> {
        parse_huge_result_copy(&mut self.buf, &mut self.cur_val);
        self.cur_val.as_ref().ok()
    }
    fn next_copy_return(&mut self) -> Option<&Huge> {
        self.cur_val = parse_huge_result_copy_return(&mut self.buf);
        self.cur_val.as_ref().ok()
    }
}

fn parse_huge(src: &mut &[u8], dest: &mut Huge) -> Result<(), Error> {
    if src.len() < DATA_SIZE { return Err(Error::new(ErrorKind::UnexpectedEof, "OH NO")) }

    (&mut dest.data[..]).write_all(&src[..DATA_SIZE])?;
    *src = &src[DATA_SIZE..];
    Ok(())
}

fn parse_huge_result(src: &mut &[u8], dest: &mut Result<Huge, Error>) {
    if src.len() < DATA_SIZE {
        *dest = Err(Error::new(ErrorKind::UnexpectedEof, "OH NO"));
        return;
    }

    let mut result = Ok(());
    if let Ok(ref mut val) = *dest {
        result = (&mut val.data[..]).write_all(&src[..DATA_SIZE]);
        *src = &src[DATA_SIZE..];
    }
    if let Err(e) = result {
        *dest = Err(e);
    }
}

fn parse_huge_result_copy(src: &mut &[u8], dest: &mut Result<Huge, Error>) {
    if src.len() < DATA_SIZE {
        *dest = Err(Error::new(ErrorKind::UnexpectedEof, "OH NO"));
        return;
    }

    unsafe {
        let mut val = Huge { data: ::std::mem::uninitialized() };
        let result = (&mut val.data[..]).write_all(&src[..DATA_SIZE]);
        if let Err(e) = result {
            *dest = Err(e);
        } else {
            *dest = Ok(val);
        }
        *src = &src[DATA_SIZE..];
    }
}

fn parse_huge_result_copy_return(src: &mut &[u8]) -> Result<Huge, Error> {
    if src.len() < DATA_SIZE { return Err(Error::new(ErrorKind::UnexpectedEof, "OH NO")); }

    unsafe {
        let mut val = Huge { data: ::std::mem::uninitialized() };
        (&mut val.data[..]).write_all(&src[..DATA_SIZE])?;
        *src = &src[DATA_SIZE..];
        Ok(val)
    }
}

#[bench]
fn bench_huge(b: &mut test::Bencher) {
    let data = test::black_box(vec![0; 1_000_000]);

    b.iter(|| {
        let mut iter = HugeIter { cur_val: Huge { data: [0; 200] }, buf: &data };
        let mut total: u8 = 0;
        while let Some(val) = iter.next() {
            total += val.data[..].iter().cloned().sum::<u8>();
        }

        total
    });
}

#[bench]
fn bench_huge_roundtrip_caller(b: &mut test::Bencher) {
    let data = test::black_box(vec![0; 1_000_000]);

    b.iter(|| {
        let mut iter = HugeIter { cur_val: Huge { data: [0; 200] }, buf: &data };
        let mut total: u8 = 0;
        while let Some(val) = iter.next_roundtrip_caller() {
            total += val.data[..].iter().cloned().sum::<u8>();
        }

        total
    });
}

#[bench]
fn bench_huge_result(b: &mut test::Bencher) {
    let data = test::black_box(vec![0; 1_000_000]);

    b.iter(|| {
        let mut iter = HugeResultIter { cur_val: Ok(Huge { data: [0; 200] }), buf: &data };
        let mut total: u8 = 0;
        while let Some(val) = iter.next() {
            total += val.data[..].iter().cloned().sum::<u8>();
        }

        total
    });
}

#[bench]
fn bench_huge_result_copy(b: &mut test::Bencher) {
    let data = test::black_box(vec![0; 1_000_000]);

    b.iter(|| {
        let mut iter = HugeResultIter { cur_val: Ok(Huge { data: [0; 200] }), buf: &data };
        let mut total: u8 = 0;
        while let Some(val) = iter.next_copy() {
            total += val.data[..].iter().cloned().sum::<u8>();
        }

        total
    });
}

#[bench]
fn bench_huge_result_copy_return(b: &mut test::Bencher) {
    let data = test::black_box(vec![0; 1_000_000]);

    b.iter(|| {
        let mut iter = HugeResultIter { cur_val: Ok(Huge { data: [0; 200] }), buf: &data };
        let mut total: u8 = 0;
        while let Some(val) = iter.next_copy_return() {
            total += val.data[..].iter().cloned().sum::<u8>();
        }

        total
    });
}
