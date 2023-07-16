
[01:07:14] ---- ssh_something_happens stdout ----
[01:07:14] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
[01:07:14] thread 'ssh_something_happens' panicked at '
[01:07:14] Expected: execs
[01:07:14]     but: expected to find:
[01:07:14] Caused by:
[01:07:14]   [[..]] Failed to start SSH session: Failed getting banner
[01:07:14] 
[01:07:14] 
[01:07:14] did not find in output:
[01:07:14]     Updating git repository `ssh://127.0.0.1:45463/foo/bar`
[01:07:14] error: failed to load source for a dependency on `bar`
[01:07:14] 
[01:07:14] Caused by:
[01:07:14]   Unable to update ssh://127.0.0.1:45463/foo/bar
[01:07:14] 
[01:07:14] Caused by:
[01:07:14]   failed to clone into: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1/home/.cargo/git/db/bar-5b585494ba4e14e6
[01:07:14] 
[01:07:14] Caused by:
[01:07:14]   [23/-1] failed to start SSH session: Failed getting banner
[01:07:14] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31
[01:07:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:14] 
[01:07:14] 
[01:07:14] failures:
[01:07:14]     ssh_something_happens
