 nocode
error: casting `&f64` as `i16` is invalid
 --> file3.rs:2:35
  |
2 |     vec![0.0].iter().map(|s| s as i16).collect::<Vec<i16>>();
  |                              -    ^^^ casting `&f64` as `i16` is invalid
  |                              |
  |                              try dereferencing for the cast to work

error: casting `&std::string::String` as `usize` is invalid
 --> file3.rs:3:13
  |
3 |     let _ = &String::new() as usize;
  |             ^^^^^^^^^^^^^^^^^^^^^^^ cast through a raw pointer first

error: aborting due to 2 previous errors
