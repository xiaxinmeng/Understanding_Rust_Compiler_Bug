plain
2019-09-11T15:07:37.3421778Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T15:07:37.3601101Z ##[command]git config gc.auto 0
2019-09-11T15:07:37.3681838Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T15:07:37.3735069Z ##[command]git config --get-all http.proxy
2019-09-11T15:07:37.3904026Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64316/merge:refs/remotes/pull/64316/merge
---
2019-09-11T15:12:12.3948558Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-09-11T15:12:16.1255542Z error: unused variable: `cmd`
2019-09-11T15:12:16.1257475Z     --> src/bootstrap/builder.rs:1340:26
2019-09-11T15:12:16.1257976Z      |
2019-09-11T15:12:16.1258562Z 1340 |     fn new(target: &str, cmd: &mut Command) -> Rustflags {
2019-09-11T15:12:16.1258866Z      |                          ^^^ help: consider prefixing with an underscore: `_cmd`
2019-09-11T15:12:16.1259630Z      = note: `-D unused-variables` implied by `-D warnings`
2019-09-11T15:12:16.1259836Z 
2019-09-11T15:12:18.0219761Z error: aborting due to previous error
2019-09-11T15:12:18.0220873Z 
---
2019-09-11T15:12:19.3404833Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-09-11T15:12:23.0370615Z error: unused variable: `cmd`
2019-09-11T15:12:23.0376812Z     --> src/bootstrap/builder.rs:1340:26
2019-09-11T15:12:23.0376901Z      |
2019-09-11T15:12:23.0377597Z 1340 |     fn new(target: &str, cmd: &mut Command) -> Rustflags {
2019-09-11T15:12:23.0377691Z      |                          ^^^ help: consider prefixing with an underscore: `_cmd`
2019-09-11T15:12:23.0378149Z      = note: `-D unused-variables` implied by `-D warnings`
2019-09-11T15:12:23.0378198Z 
2019-09-11T15:12:24.9643714Z error: aborting due to previous error
2019-09-11T15:12:24.9644667Z 
---
2019-09-11T15:12:27.2639409Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-09-11T15:12:30.9321346Z error: unused variable: `cmd`
2019-09-11T15:12:30.9323357Z     --> src/bootstrap/builder.rs:1340:26
2019-09-11T15:12:30.9323816Z      |
2019-09-11T15:12:30.9324440Z 1340 |     fn new(target: &str, cmd: &mut Command) -> Rustflags {
2019-09-11T15:12:30.9324819Z      |                          ^^^ help: consider prefixing with an underscore: `_cmd`
2019-09-11T15:12:30.9325685Z      = note: `-D unused-variables` implied by `-D warnings`
2019-09-11T15:12:30.9326007Z 
2019-09-11T15:12:32.8373699Z error: aborting due to previous error
2019-09-11T15:12:32.8374718Z 
---
2019-09-11T15:12:36.1581899Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-09-11T15:12:39.8758658Z error: unused variable: `cmd`
2019-09-11T15:12:39.8760393Z     --> src/bootstrap/builder.rs:1340:26
2019-09-11T15:12:39.8760635Z      |
2019-09-11T15:12:39.8761098Z 1340 |     fn new(target: &str, cmd: &mut Command) -> Rustflags {
2019-09-11T15:12:39.8761322Z      |                          ^^^ help: consider prefixing with an underscore: `_cmd`
2019-09-11T15:12:39.8761952Z      = note: `-D unused-variables` implied by `-D warnings`
2019-09-11T15:12:39.8762131Z 
2019-09-11T15:12:41.8285026Z error: aborting due to previous error
2019-09-11T15:12:41.8286202Z 
---
2019-09-11T15:12:46.1340626Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-09-11T15:12:49.7995492Z error: unused variable: `cmd`
2019-09-11T15:12:49.7997590Z     --> src/bootstrap/builder.rs:1340:26
2019-09-11T15:12:49.7998157Z      |
2019-09-11T15:12:49.7998860Z 1340 |     fn new(target: &str, cmd: &mut Command) -> Rustflags {
2019-09-11T15:12:49.7999553Z      |                          ^^^ help: consider prefixing with an underscore: `_cmd`
2019-09-11T15:12:49.8000546Z      = note: `-D unused-variables` implied by `-D warnings`
2019-09-11T15:12:49.8000903Z 
2019-09-11T15:12:51.7253583Z error: aborting due to previous error
2019-09-11T15:12:51.7254707Z 
---
2019-09-11T15:12:51.7861470Z == clock drift check ==
2019-09-11T15:12:51.7874594Z   local time: Wed Sep 11 15:12:51 UTC 2019
2019-09-11T15:12:51.9366232Z   network time: Wed, 11 Sep 2019 15:12:51 GMT
2019-09-11T15:12:51.9370744Z == end clock drift check ==
2019-09-11T15:12:52.8137306Z ##[error]Bash exited with code '1'.
2019-09-11T15:12:52.8169300Z ##[section]Starting: Checkout
2019-09-11T15:12:52.8171441Z ==============================================================================
2019-09-11T15:12:52.8171507Z Task         : Get sources
2019-09-11T15:12:52.8171579Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
