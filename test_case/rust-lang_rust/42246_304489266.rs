
[01:06:02]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/build_auth-681d1f230bae11b8
[01:06:02] 
[01:06:02] running 3 tests
[01:06:02] test ssh_something_happens ... FAILED
[01:06:02] test https_something_happens ... ok
[01:06:02] test http_auth_offered ... ok
[01:06:02] 
[01:06:02] failures:
[01:06:02] 
[01:06:02] ---- ssh_something_happens stdout ----
[01:06:02] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
[01:06:02] thread 'ssh_something_happens' panicked at '
[01:06:02] Expected: execs
[01:06:02]     but: expected to find:
[01:06:02] Caused by:
[01:06:02]   [[..]] Failed to start SSH session: Failed getting banner
[01:06:02] 
[01:06:02] 
[01:06:02] did not find in output:
[01:06:02]     Updating git repository `ssh://127.0.0.1:37962/foo/bar`
[01:06:02] error: failed to load source for a dependency on `bar`
[01:06:02] 
[01:06:02] Caused by:
[01:06:02]   Unable to update ssh://127.0.0.1:37962/foo/bar
[01:06:02] 
[01:06:02] Caused by:
[01:06:02]   failed to clone into: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t2/home/.cargo/git/db/bar-66fdece68bd893fc
[01:06:02] 
[01:06:02] Caused by:
[01:06:02]   [23/-1] failed to start SSH session: Failed getting banner
[01:06:02] ', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31
[01:06:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
