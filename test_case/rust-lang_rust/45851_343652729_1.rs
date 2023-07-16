rust
#![feature(test)]
extern crate test;
extern crate tempdir;

use std::fs::File;
use std::io::{Write, Read};
use test::Bencher;

fn run<F>(bencher: &mut Bencher, size: usize, new_buffer: F)
    where F: Fn(&File) -> Vec<u8>
{
    let dir = tempdir::TempDir::new("bench").unwrap();
    let path = dir.path().join("something");
    File::create(&path).unwrap().write_all(&vec![42; size]).unwrap();
    bencher.bytes = size as u64;
    bencher.iter(|| {
        let mut file = File::open(&path).unwrap();
        let mut buffer = new_buffer(&file);
        file.read_to_end(&mut buffer).unwrap();
    })
}

macro_rules! sizes {
    ($( $name: ident  $size: expr )+) => {
        $(
            mod $name {
                use super::*;

                #[bench]
                fn vec_new(bencher: &mut Bencher) {
                    run(bencher, $size, |_| Vec::new())
                }

                #[bench]
                fn vec_with_capacity(bencher: &mut Bencher) {
                    run(bencher, $size, |file| {
                        Vec::with_capacity(file.metadata().unwrap().len() as usize)
                    })
                }
            }
        )+
    }
}

sizes! {
    a_read_128 128
    b_read_512 512
    c_read_2k 2 * 1024
    d_read_8k 8 * 1024
    e_read_32k 32 * 1024
    f_read_128k 128 * 1024
    g_read_512k 512 * 1024
    h_read_1m 1024 * 1024
    i_read_2m 2 * 1024 * 1024
    j_read_4m 4 * 1024 * 1024
    k_read_8m 8 * 1024 * 1024
    l_read_32m 32 * 1024 * 1024
}
