
   Compiling playground v0.0.1 (file:///playground)
error[E0284]: type annotations required: cannot resolve `<_ as std::ops::Try>::Ok == _`
 --> src/main.rs:6:28
  |
6 |       let parsed: Vec<u32> = s.iter().map(|str| {
  |  ____________________________^
7 | |         let val = str.parse::<u32>()?;
8 | |         Ok(val + 1)
9 | |     }).collect()?;
  | |_________________^

error: aborting due to previous error
