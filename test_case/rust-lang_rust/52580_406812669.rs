rust
extern crate io;

use core::mem;
pub use self::io::*;

fn fst2<S, T>((x, _): (S, T)) -> S { x }

pub fn writeCode<Codon: Copy, F: FnMut(T, &mut [Codon]) -> Option<usize>, I: Iterator<Item = T>, T, W: Write<Codon>>
                (mut encode: F, w: &mut W, xs : I) -> Result<usize, W::Err> where W::Err: From<EndOfFile> {
    let mut buf: [Codon; 4096] = unsafe { mem::uninitialized() };
    let mut pos = 0;
    let mut nBytesWritten = 0;
    for x in xs {
        match encode(x, &mut buf[pos..]) {
            Some(n) => { pos += n; },
            None => {
                try!(w.write_all(&buf[0..pos]).map_err(fst2));
                nBytesWritten += pos;
                pos = 0;
            }
        }
    }
    try!(w.write_all(&buf[0..pos]).map_err(fst2));
    nBytesWritten += pos;
    Ok(nBytesWritten)
}
