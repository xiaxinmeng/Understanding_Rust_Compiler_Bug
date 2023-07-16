
   Compiling playground v0.0.1 (/playground)
error[E0521]: borrowed data escapes outside of closure
 --> src/main.rs:3:33
  |
2 |     let mut stash: Option<&i32> = None;
  |         --------- `stash` declared here, outside of the closure body
3 |     let mut thief = |r: &i32| { stash = Some(r) };
  |                      -          ^^^^^^^^^^^^^^^ `r` escapes the closure body here
  |                      |
  |                      `r` is a reference that is only valid in the closure body
