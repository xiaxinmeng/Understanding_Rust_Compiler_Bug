 rust
use std::error::Error;
use std::thread;
use std::time::Duration;

use brotli::{BrotliDecompressCustomAlloc, reader::StandardAlloc};

use std::fs::File;
use std::io::Read;
use std::mem;

fn from_file(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(file_path)?;
    // Emitating fs::read()
    let size = f.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
    let mut bytes = Vec::with_capacity(size);
    f.read_to_end(&mut bytes)?;
    let mut in_buf  = unsafe { mem::uninitialized::<[u8; 256*1024]>() };
    let mut out_buf = unsafe { mem::uninitialized::<[u8; 256*1024]>() };
    let mut decompressed = Vec::with_capacity(bytes.len()*16);
    let alloc = || StandardAlloc::default();
    BrotliDecompressCustomAlloc(&mut &*bytes, &mut decompressed, &mut in_buf, &mut out_buf, alloc(), alloc(), alloc())?;
    Ok(decompressed)
}

fn main() {
    let file_path = std::env::args().nth(1).expect("No file path arg passed");
    for i in 1..=8 {
        // Running process should be occupying almost the same small amount of memory here, no?
        eprintln!("sleeping {}", i);
        thread::sleep(Duration::new(3, 0));
        eprintln!("resuming");
        let _bytes = from_file(&file_path).unwrap();
    }
    eprintln!("sleeping for 1 minute");
    thread::sleep(Duration::new(60, 0));
}
