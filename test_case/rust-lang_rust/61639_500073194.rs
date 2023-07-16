plain
travis_time:end:21bd8900:start=1559945066405219453,finish=1559945164599846864,duration=98194627411
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:45] 
[01:09:45] running 144 tests
[01:09:48] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:09:50] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:09:50] 
[01:09:50]  finished in 4.747
[01:09:50] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:52] 
[01:09:52] running 9 tests
[01:09:52] iiiiiiiii
[01:09:52] 
[01:09:52]  finished in 0.157
[01:09:52] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:08] 
[01:10:08] running 122 tests
[01:10:34] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:10:39] .i.i......iii.i.....ii
[01:10:39] 
[01:10:39]  finished in 31.122
[01:10:39] travis_fold:end:test_debuginfo

---
[01:50:36] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:50:36] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:50:36] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:50:36] 
[01:50:36] Diff < left / right > :
[01:50:36]      Std {
[01:50:36]          target: "A",
[01:50:36]          compiler: Compiler {
[01:50:36]              stage: 0,
---
[01:50:36]              stage: 1,
[01:50:36]              host: "A",
[01:50:36]          },
[01:50:36]      },
[01:50:36] >    Std {
[01:50:36] >        target: "B",
[01:50:36] >        compiler: Compiler {
[01:50:36] >            stage: 2,
[01:50:36] >            host: "A",
[01:50:36] >        },
[01:50:36] >    },
[01:50:36]  ]
[01:50:36] ', src/bootstrap/builder.rs:1677:9
[01:50:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:50:36] 
[01:50:36] 
