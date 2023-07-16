 rust
use std::slice::Chunks;
use std::slice::MutChunks;

fn dft_iter<'a, T>(arg1: Chunks<'a,T>, mut arg2: MutChunks<'a,T>)
{
    for &something in arg2
    {
    }
}

fn main() { }
