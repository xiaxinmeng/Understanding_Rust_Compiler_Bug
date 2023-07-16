
[01:05:59]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/concurrent-4e888deb515d5802
[01:05:59] 
[01:05:59] running 10 tests
[01:06:00] test concurrent_installs ... FAILED
[01:06:00] test git_same_repo_different_tags ... ok
[01:06:01] test debug_release_ok ... ok
[01:06:02] test multiple_installs ... ok
[01:06:03] test git_same_branch_different_revs ... ok
[01:06:04] test killing_cargo_releases_the_lock ... ok
[01:06:04] test no_deadlock_with_git_dependencies ... ok
[01:06:04] test one_install_should_be_bad ... ok
[01:06:05] test multiple_registry_fetches ... ok
[01:06:05] test same_project ... ok
[01:06:05] 
[01:06:05] failures:
[01:06:05] 
[01:06:05] ---- concurrent_installs stdout ----
[01:06:05] 	thread 'concurrent_installs' panicked at '
[01:06:05] Expected: execs
[01:06:05]     but: exited with exit code: 101
[01:06:05] --- stdout
[01:06:05] 
[01:06:05] --- stderr
[01:06:05]     Updating registry `file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t0/registry`
[01:06:05]     Blocking waiting for file lock on the registry index
[01:06:05] error: could not find `foo` in `registry https://github.com/rust-lang/crates.io-index`
[01:06:05] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31
[01:06:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:05] 
[01:06:05] 
[01:06:05] failures:
[01:06:05]     concurrent_installs
[01:06:05] 
[01:06:05] test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured
[01:06:05] 
[01:06:05] error: test failed, to rerun pass '--test concurrent'
