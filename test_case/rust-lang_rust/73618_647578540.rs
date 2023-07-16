
2020-06-22T14:58:34.7710007Z Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
2020-06-22T14:58:34.9722276Z     Finished release [optimized] target(s) in 0.19s
2020-06-22T14:58:34.9948392Z Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> armv5te-unknown-linux-gnueabi)
2020-06-22T14:58:35.1711736Z 
2020-06-22T14:58:35.1712134Z running 110 tests
2020-06-22T14:58:43.6758586Z ..............................................................i..................................... 100/110
2020-06-22T14:58:44.4790181Z ..........
2020-06-22T14:58:44.4791879Z test result: ok. 109 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
2020-06-22T14:58:44.4792098Z 
2020-06-22T14:58:44.4797973Z 	finished in 9.485
2020-06-22T14:58:44.4852548Z Build completed successfully in 0:01:16
2020-06-22T14:58:44.4861227Z + python2.7 ../x.py test src/tools/tidy
2020-06-22T14:58:44.7099048Z     Finished dev [unoptimized] target(s) in 0.17s
2020-06-22T14:58:44.8789960Z Building stage0 tool tidy (x86_64-unknown-linux-gnu)
2020-06-22T14:58:45.0540868Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-06-22T14:58:45.6805014Z     Finished release [optimized] target(s) in 0.80s
2020-06-22T14:58:45.6911437Z tidy check
2020-06-22T14:58:48.5364510Z * 607 error codes
2020-06-22T14:58:48.5365253Z * highest error code: E0764
2020-06-22T14:58:48.9013583Z * 291 features
2020-06-22T14:58:49.8030201Z thread 'main' panicked at 'cmd.exec() failed with Error during execution of `cargo metadata`:  Downloading crates ...
2020-06-22T14:58:49.8031384Z warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/futures/0.1.28/download`, got 503
2020-06-22T14:58:49.8032007Z warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/futures/0.1.28/download`, got 503
2020-06-22T14:58:49.8032528Z error: failed to download from `https://crates.io/api/v1/crates/futures/0.1.28/download`
2020-06-22T14:58:49.8032780Z 
2020-06-22T14:58:49.8032900Z Caused by:
2020-06-22T14:58:49.8033204Z   failed to get 200 response from `https://crates.io/api/v1/crates/futures/0.1.28/download`, got 503
2020-06-22T14:58:49.8033804Z ', src/tools/tidy/src/deps.rs:197:20
2020-06-22T14:58:49.8034286Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-06-22T14:58:49.8037305Z 
2020-06-22T14:58:49.8037473Z 
2020-06-22T14:58:49.8038772Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-06-22T14:58:49.8039211Z expected success, got: exit code: 101
2020-06-22T14:58:49.8039350Z 
2020-06-22T14:58:49.8039426Z 
2020-06-22T14:58:49.8042832Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-06-22T14:58:49.8043160Z Build completed unsuccessfully in 0:00:05
2020-06-22T14:58:49.8089498Z == clock drift check ==
2020-06-22T14:58:49.8109861Z   local time: Mon Jun 22 14:58:49 UTC 2020
2020-06-22T14:58:49.9848862Z   network time: Mon, 22 Jun 2020 14:58:49 GMT
2020-06-22T14:58:49.9853495Z == end clock drift check ==
2020-06-22T14:58:52.9605408Z 
2020-06-22T14:58:52.9679961Z ##[error]Bash exited with code '1'.
2020-06-22T14:58:52.9693855Z ##[section]Finishing: Run build
