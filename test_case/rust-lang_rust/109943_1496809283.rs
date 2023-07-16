rs
#![feature(rustc_attrs)]

use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::*;


#[rustc_mir(borrowck_graphviz_postflow="suffix.dot")]
fn main() {
    let _ = Interval::new(Instant::now(), Duration::from_secs(1))
        .map(|_| ())
        .for_each(Ok);
}
