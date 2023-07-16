rust

extern crate rayon;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::File;
use rayon::prelude::*;

fn main() {

    let file =  File::open("file").unwrap();
        
    [0,1].par_iter()
        .for_each(|p| {
            let mut bytes = vec![];
            bytes.resize(8, 0);
            load_bytes(&mut bytes, &file, *p as u64);
        });

}

fn load_bytes(buffer: &mut Vec<u8>, mut file: &File, offset: u64) {
    file.seek(SeekFrom::Start(offset)).unwrap();
    file.read_exact(buffer).unwrap();
}
