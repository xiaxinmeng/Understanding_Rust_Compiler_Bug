
[01:08:29] failures:
[01:08:29] 
[01:08:29] ---- frozen_flag_preserves_old_lockfile stdout ----
[01:08:29] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build --locked`
[01:08:29] thread 'frozen_flag_preserves_old_lockfile' panicked at '
[01:08:29] Expected: execs
[01:08:29]     but: exited with exit code: 101
[01:08:29] --- stdout
[01:08:29] 
[01:08:29] --- stderr
[01:08:29]     Updating registry `file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1/registry`
[01:08:29] error: checksum for `foo v0.1.0` changed between lock files
[01:08:29] 
[01:08:29] this could be indicative of a few possible errors:
[01:08:29] 
[01:08:29]     * the lock file is corrupt
[01:08:29]     * a replacement source in use (e.g. a mirror) returned a different checksum
[01:08:29]     * the source itself may be corrupt in one way or another
[01:08:29] 
[01:08:29] unable to verify that `foo v0.1.0` is the same as when the lockfile was generated
[01:08:29] 
[01:08:29] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31:12
[01:08:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:29] 
[01:08:29] 
[01:08:29] failures:
[01:08:29]     frozen_flag_preserves_old_lockfile
[01:08:29] 
[01:08:29] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:29] 
[01:08:29] [m[m[31m[1merror:[m test failed, to rerun pass '--test lockfile-compat'
[01:08:29] 
[01:08:29] 
[01:08:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml"
