plain
travis_time:end:01aba4ae:start=1552389707315671630,finish=1552389783287767364,duration=75972095734
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:43] 
[01:25:43] running 120 tests
[01:26:11] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:26:16] .i......iii.i.....ii
[01:26:16] 
[01:26:16]  finished in 33.806
[01:26:16] travis_fold:end:test_debuginfo

---
[01:46:43] ..........iii......i......i...i......i.............................................................. 300/996
[01:46:48] .................................................................................................... 400/996
[01:46:57] ........................i.i.....................................iiii........ii...................... 500/996
[01:47:04] .................................................................................................... 600/996
[01:47:12] ..FF................................................................................................ 700/996
[01:47:36] .................................................................................................... 900/996
[01:47:43] ..........................................iiii..................................................
[01:47:43] failures:
[01:47:43] 
[01:47:43] 
[01:47:43] ---- net/udp.rs - net::udp::UdpSocket::peer_addr (line 198) stdout ----
[01:47:43] error: unused imports: `Ipv4Addr`, `SocketAddrV4`, `SocketAddr`
[01:47:43]  --> net/udp.rs:199:16
[01:47:43]   |
[01:47:43] 4 | use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
[01:47:43]   |
[01:47:43] note: lint level defined here
[01:47:43]  --> net/udp.rs:196:9
[01:47:43]   |
[01:47:43]   |
[01:47:43] 1 | #![deny(warnings)]
[01:47:43]   |         ^^^^^^^^
[01:47:43]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:47:43] 
[01:47:43] error[E0658]: use of unstable library feature 'udp_peer_addr' (see issue #59127)
[01:47:43]  --> net/udp.rs:202:19
[01:47:43]   |
[01:47:43] 7 | assert_eq!(socket.peer_addr().unwrap_err().kind(),
[01:47:43]   |
[01:47:43]   |
[01:47:43]   = help: add #![feature(udp_peer_addr)] to the crate attributes to enable
[01:47:43] error: aborting due to 2 previous errors
[01:47:43] 
[01:47:43] For more information about this error, try `rustc --explain E0658`.
[01:47:43] thread 'net/udp.rs - net::udp::UdpSocket::peer_addr (line 198)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:47:43] thread 'net/udp.rs - net::udp::UdpSocket::peer_addr (line 198)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:47:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:47:43] 
[01:47:43] ---- net/udp.rs - net::udp::UdpSocket::peer_addr (line 189) stdout ----
[01:47:43] error[E0658]: use of unstable library feature 'udp_peer_addr' (see issue #59127)
[01:47:43]  --> net/udp.rs:194:19
[01:47:43]   |
[01:47:43] 8 | assert_eq!(socket.peer_addr().unwrap(),
[01:47:43]   |
[01:47:43]   |
[01:47:43]   = help: add #![feature(udp_peer_addr)] to the crate attributes to enable
[01:47:43] error: aborting due to previous error
[01:47:43] 
[01:47:43] For more information about this error, try `rustc --explain E0658`.
[01:47:43] thread 'net/udp.rs - net::udp::UdpSocket::peer_addr (line 189)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:47:43] 
[01:47:43] error: test failed, to rerun pass '--doc'
[01:47:43] 
[01:47:43] 
[01:47:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:47:43] 
[01:47:43] 
[01:47:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:43] Build completed unsuccessfully in 0:35:34
[01:47:43] Build completed unsuccessfully in 0:35:34
[01:47:43] make: *** [check] Error 1
[01:47:43] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01f13820
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 13:10:57 UTC 2019
---
travis_time:end:0029d224:start=1552396259325954327,finish=1552396259331071461,duration=5117134
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e3cebc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18e4b66c
travis_time:start:18e4b66c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2cb01576
$ dmesg | grep -i kill
