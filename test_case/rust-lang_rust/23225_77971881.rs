 rust
use std::marker::MarkerTrait;

trait Test: MarkerTrait {}

impl Test for .. {}

fn main() {}
