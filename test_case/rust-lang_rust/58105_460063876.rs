plain
travis_time:end:0f48edc5:start=1549204209406195442,finish=1549204281582192805,duration=72175997363
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:18] 
[01:09:18] running 119 tests
[01:09:42] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:46] i......iii.i.....ii
[01:09:46] 
[01:09:46]  finished in 28.688
[01:09:46] travis_fold:end:test_debuginfo

---
[01:28:50]    Compiling arena v0.0.0 (/checkout/src/libarena)
[01:28:50] error: hidden lifetime parameters in types are deprecated
[01:28:50]    --> src/libarena/lib.rs:516:25
[01:28:50]     |
[01:28:50] 516 |                 let r: &EI = self.0.alloc(EI::I(f()));
[01:28:50] 
[01:28:50] error: hidden lifetime parameters in types are deprecated
[01:28:50]    --> src/libarena/lib.rs:524:25
[01:28:50]     |
[01:28:50]     |
[01:28:50] 524 |                 let r: &EI = self.0.alloc(EI::O(f()));
[01:28:50] 
[01:28:50] error: hidden lifetime parameters in types are deprecated
[01:28:50]    --> src/libarena/lib.rs:523:67
[01:28:50]     |
[01:28:50]     |
[01:28:50] 523 |             fn alloc_outer<F: Fn() -> Outer<'a>>(&self, f: F) -> &Outer {
[01:28:50] 
[01:28:50] error: hidden lifetime parameters in types are deprecated
[01:28:50]    --> src/libarena/lib.rs:624:35
[01:28:50]     |
[01:28:50]     |
[01:28:50] 624 |             let arena: TypedArena<DropCounter> = TypedArena::default();
[01:28:50] 
[01:28:50] error: hidden lifetime parameters in types are deprecated
[01:28:50]    --> src/libarena/lib.rs:636:35
[01:28:50]     |
[01:28:50]     |
[01:28:50] 636 |         let mut arena: TypedArena<DropCounter> = TypedArena::default();
[01:28:50] 
[01:28:50] error: aborting due to 5 previous errors
[01:28:50] 
[01:28:50] error: Could not compile `arena`.
[01:28:50] error: Could not compile `arena`.
[01:28:50] 
[01:28:50] To learn more, run the command again with --verbose.
[01:28:50] 
[01:28:50] 
[01:28:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "arena" "--" "--quiet"
[01:28:50] 
[01:28:50] 
[01:28:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:50] Build completed unsuccessfully in 0:31:02
[01:28:50] Build completed unsuccessfully in 0:31:02
[01:28:50] Makefile:48: recipe for target 'check' failed
[01:28:50] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0722ab76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 16:00:21 UTC 2019
