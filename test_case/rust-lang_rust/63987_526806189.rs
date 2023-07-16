text
warning: some ranges overlap
 --> src/main.rs:3:9
  |
3 |         1 ..= 8 => {}
  |         ^^^^^^^
  |
  = note: `#[warn(clippy::match_overlapping_arm)]` on by default
note: overlaps with this
 --> src/main.rs:4:9
  |
4 |         5 ..= 20 => {} // This should probably have been `8 ..= 20`
  |         ^^^^^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_overlapping_arm
