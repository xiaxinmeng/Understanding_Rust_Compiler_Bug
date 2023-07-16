plain
2019-09-18T21:36:06.6551335Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T21:36:06.6807985Z ##[command]git config gc.auto 0
2019-09-18T21:36:06.6882928Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T21:36:06.6934603Z ##[command]git config --get-all http.proxy
2019-09-18T21:36:06.7074384Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63934/merge:refs/remotes/pull/63934/merge
---
2019-09-18T21:44:34.5299432Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-18T21:45:01.2731008Z error: unreachable pattern
2019-09-18T21:45:01.2731601Z    --> src/librustc/traits/coherence.rs:548:9
2019-09-18T21:45:01.2731882Z     |
2019-09-18T21:45:01.2732194Z 548 |         ty::Opaque(..) => {
2019-09-18T21:45:01.2732789Z     |
2019-09-18T21:45:01.2733104Z     = note: `-D unreachable-patterns` implied by `-D warnings`
2019-09-18T21:45:01.2733179Z 
2019-09-18T21:45:22.0082068Z error: aborting due to previous error
---
2019-09-18T21:45:22.3674739Z == clock drift check ==
2019-09-18T21:45:22.3690945Z   local time: Wed Sep 18 21:45:22 UTC 2019
2019-09-18T21:45:22.6357875Z   network time: Wed, 18 Sep 2019 21:45:22 GMT
2019-09-18T21:45:22.6362025Z == end clock drift check ==
2019-09-18T21:45:23.2806325Z ##[error]Bash exited with code '1'.
2019-09-18T21:45:23.2852154Z ##[section]Starting: Checkout
2019-09-18T21:45:23.2853932Z ==============================================================================
2019-09-18T21:45:23.2853986Z Task         : Get sources
2019-09-18T21:45:23.2854034Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
