plain
[RUSTC-TIMING] addr2line test:false 0.461
[RUSTC-TIMING] core test:false 39.947
[RUSTC-TIMING] gimli test:false 5.937
[RUSTC-TIMING] object test:false 10.853
error[E0425]: cannot find value `SYS_clone3` in this scope
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.88/src/unix/linux_like/linux/musl/b64/x86_64/mod.rs:306:1
306 |   pub const SYS_clone: ::c_long = 56;
306 |   pub const SYS_clone: ::c_long = 56;
    |   ----------------------------------- similarly named constant `SYS_clone` defined here
    |
122 | / macro_rules! syscall {
122 | / macro_rules! syscall {
123 | |     (fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
124 | |         unsafe fn $name($($arg_name:$t),*) -> $ret {
125 | |             // This looks like a hack, but concat_idents only accepts idents
...   |
137 | |                     concat_idents!(SYS_, $name),
    | |                     |
    | |                     help: a constant with a similar name exists: `SYS_clone`
    | |                     in this macro invocation (#2)
...   |
...   |
142 | |     )
143 | | }
    | |_- in this expansion of `syscall!` (#1)
   ::: library/std/src/sys/unix/process/process_unix.rs:154:13
    |
154 | /             syscall! {
154 | /             syscall! {
155 | |                 fn clone3(cl_args: *mut clone_args, len: libc::size_t) -> c_long
    | |_____________- in this macro invocation (#1)

error: aborting due to previous error


For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 2.594
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-musl/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-musl" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build x86_64-unknown-linux-musl
Build completed unsuccessfully in 0:02:38
