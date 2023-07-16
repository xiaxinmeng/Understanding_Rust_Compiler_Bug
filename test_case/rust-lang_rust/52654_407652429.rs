plain
[01:57:34] warning: spurious network error (2 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:57:34] ; class=Net (12)
[01:58:30] warning: spurious network error (1 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:58:30] ; class=Net (12)
[01:59:26] error: failed to load source for a dependency on `rand`
[01:59:26] Caused by:
[01:59:26]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[01:59:26] 
[01:59:26] Caused by:
[01:59:26] Caused by:
[01:59:26]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[01:59:26] 
[01:59:26] Caused by:
[01:59:26]   curl error: Couldn't resolve host 'github.com'
[01:59:26] ; class=Net (12)
[01:59:26] 
[01:59:26] 
[01:59:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[01:59:26] 
[01:59:26] 
[01:59:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[01:59:26] Build completed unsuccessfully in 1:56:49
