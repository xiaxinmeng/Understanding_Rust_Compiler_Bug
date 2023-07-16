
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    --> /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/sys/windows/compat.rs:254:21
     |
254  |                       Module::new($module).map(preload);
     |                       ^^^^^^^^^^^^^^^^^^^^ call to unsafe function
     |
    ::: /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/std/src/sys/windows/c.rs:1247:1
     |
1247 | / compat_fn_optional! {
1248 | |     pub static SYNCH_API: &CStr = ansi_str!("api-ms-win-core-synch-l1-2-0");
1249 | |
1250 | |     // >= Windows 8 / Server 2012
...    |
1258 | |     pub fn WakeByAddressSingle(Address: LPVOID) -> ();
1259 | | }
     | |_- in this macro invocation
