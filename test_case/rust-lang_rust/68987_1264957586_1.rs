
error: unexpected closing delimiter: `}`
  --> src/main.rs:36:1
   |
32 |     } else {
   |            - this opening brace...
33 |         Err(MyError { name: "test" })
34 |     }
   |     - ...matches this closing brace
35 |
36 | }
   | ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
  --> src/main.rs:20:55
   |
20 | async fn obstest() -> Result<impl Responder, MyError> {
   |                                                       ^ unclosed delimiter
21 |     let obs_connect = || -> Result<(obws::Version, Vec<obws::responses::scenes::Scene>)>, MyError) {
   |                                                                                                  ^ mismatched closing delimiter

error: could not compile `scuffcommander` due to 2 previous errors
