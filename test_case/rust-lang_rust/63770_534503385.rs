plain
2019-09-24T10:48:04.4327844Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-09-24T10:48:04.7436576Z error[E0133]: access to union field is unsafe and requires unsafe function or block
2019-09-24T10:48:04.7437069Z   --> src/libcore/slice/mod.rs:70:13
2019-09-24T10:48:04.7437752Z    |
2019-09-24T10:48:04.7438101Z 70 |             crate::ptr::Repr { rust: self }.raw.len
2019-09-24T10:48:04.7438712Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ access to union field
2019-09-24T10:48:04.7439000Z    |
2019-09-24T10:48:04.7439347Z    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
2019-09-24T10:48:04.7471984Z error[E0133]: access to union field is unsafe and requires unsafe function or block
2019-09-24T10:48:04.7472326Z     --> src/libcore/str/mod.rs:2180:18
2019-09-24T10:48:04.7472545Z      |
2019-09-24T10:48:04.7472545Z      |
2019-09-24T10:48:04.7472880Z 2180 |         unsafe { Slices { str: self }.slice }
2019-09-24T10:48:04.7473252Z      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^ access to union field
2019-09-24T10:48:04.7473490Z      |
2019-09-24T10:48:04.7474038Z      = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
2019-09-24T10:48:04.7519607Z error: aborting due to 2 previous errors
2019-09-24T10:48:04.7519742Z 
2019-09-24T10:48:04.7528249Z For more information about this error, try `rustc --explain E0133`.
2019-09-24T10:48:04.8476969Z [RUSTC-TIMING] build_script_build test:false 0.410
---
2019-09-24T10:48:05.3772048Z == clock drift check ==
2019-09-24T10:48:05.3784430Z   local time: Tue Sep 24 10:48:05 UTC 2019
2019-09-24T10:48:05.5310826Z   network time: Tue, 24 Sep 2019 10:48:05 GMT
2019-09-24T10:48:05.5317046Z == end clock drift check ==
2019-09-24T10:48:09.6491482Z ##[error]Bash exited with code '1'.
2019-09-24T10:48:09.6527647Z ##[section]Starting: Upload CPU usage statistics
2019-09-24T10:48:09.6531121Z ==============================================================================
2019-09-24T10:48:09.6531198Z Task         : Bash
2019-09-24T10:48:09.6531281Z Description  : Run a Bash script on macOS, Linux, or Windows
