
[00:41:38] Linkcheck (x86_64-unknown-linux-gnu)
[00:41:41] std/ffi/index.html:73: broken link - std/ffi/struct.CStr
[00:41:41] std/ffi/index.html:81: broken link - std/ffi/struct.CString
[00:41:41] std/ffi/index.html:89: broken link - std/ffi/struct.CString
[00:41:41] std/ffi/struct.FromBytesWithNulError.html:52: broken link - std/ffi/struct.CStr
[00:41:41] std/ffi/struct.NulError.html:52: broken link - std/ffi/struct.CString
[00:41:41] std/ffi/struct.IntoStringError.html:52: broken link - std/ffi/struct.CString
[00:41:43] thread 'main' panicked at 'found some broken links', /checkout/src/tools/linkchecker/main.rs:49
[00:41:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:41:43] 
[00:41:43] 
[00:41:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[00:41:43] expected success, got: exit code: 101
