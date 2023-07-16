rust
#[derive(Copy,Clone)] struct ZST;
let x = [ZST; usize::max_value()];
let y = x.iter().count();
