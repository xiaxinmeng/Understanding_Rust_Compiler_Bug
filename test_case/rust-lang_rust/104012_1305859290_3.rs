console
error: unexpected closing delimiter: `}`
  --> bug2.rs:12:1
   |
12 | }
   | ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
 --> bug2.rs:1:32
  |
1 | async fn obstest() -> Result<> {
  |                                ^ unclosed delimiter
2 |     let obs_connect = || -> Result<(), MyError) {
  |                                               ^ mismatched closing delimiter

error: aborting due to 2 previous errors
