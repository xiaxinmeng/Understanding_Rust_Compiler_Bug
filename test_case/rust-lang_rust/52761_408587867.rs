plain
[00:14:13] * inclusive_range_methods          lib      stable       1.27.0  
[00:14:13] * inclusive_range_syntax           lang     stable       1.26.0  
[00:14:13] * indirect_hasher_impl             lib      stable       1.22.0  
[00:14:13] * infer_outlives_requirements      lang     unstable     1.26.0  
[00:14:13] * infer_static_outlives_requirements lang     unstable     1.26.0  
[00:14:13] * int_error_internals              lib      unstable     None    
[00:14:13] * int_to_from_bytes                lib      stable       1.29.0  
[00:14:13] * integer_atomics                  lib      unstable     None    
[00:14:13] * into_boxed_c_str                 lib      stable       1.20.0  
---
[01:47:52] warning: spurious network error (2 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:47:52] ; class=Net (12)
[01:48:48] warning: spurious network error (1 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:48:48] ; class=Net (12)
[01:49:44] error: failed to load source for a dependency on `rand`
[01:49:44] Caused by:
[01:49:44]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[01:49:44] 
[01:49:44] Caused by:
[01:49:44] Caused by:
[01:49:44]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[01:49:44] 
[01:49:44] Caused by:
[01:49:44]   curl error: Couldn't resolve host 'github.com'
[01:49:44] ; class=Net (12)
[01:49:44] 
[01:49:44] 
[01:49:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[01:49:44] 
[01:49:44] 
[01:49:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[01:49:44] Build completed unsuccessfully in 1:46:57
