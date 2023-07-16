rust
error: the type of this value must be known in this context
 --> spanerror1.rs:3:26
  |
3 |       let nums: Vec<u64> = line.split_whitespace()
  |  __________________________^ starting here...
4 | |         .map(|num| num.parse())
5 | |         .collect()
6 | |         .unwrap();
  | |_________________^ ...ending here

error: aborting due to previous error
