rust
use async_std::{fs, io, prelude::*};

type Handle = io::BufWriter<fs::File>;

async fn die_horribly(thing: &[Handle]) {
    for v in thing {
        v.flush().await;
    }
}

fn main() {}
