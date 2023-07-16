plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 20ffea6938b5839c390252e07940b99e3b6a889a and 4acd250f4b2f986e9613326a03579727ce7516b5
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
     |                                  ^^^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^ expected 4 fields, found 3
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
621  |             ExprKind::MethodCall(ref method_def, ref arguments, _, _) => {
---
    = help: the trait `Sized` is not implemented for `str`
note: required by a bound in `ArgumentV1::<'a>::new_display`
   --> /checkout/library/core/src/fmt/mod.rs:318:23
    |
313 | / macro_rules! arg_new {
314 | |     ($f: ident, $t: ident) => {
315 | |         #[doc(hidden)]
316 | |         #[unstable(feature = "fmt_internals", reason = "internal to format_args!", issue = "none")]
317 | |         #[inline]
318 | |         pub fn $f<'b, T: $t>(x: &'b T) -> ArgumentV1<'_> {
    | |                       ^ required by this bound in `ArgumentV1::<'a>::new_display`
321 | |     };
322 | | }
    | |_- in this expansion of `arg_new!`
...
...
341 |       arg_new!(new_display, Display);

   Compiling clap v3.2.5
Some errors have detailed explanations: E0023, E0277.
For more information about an error, try `rustc --explain E0023`.
---
actual output differed from expected tests/pass/concurrency/simple.stderr
Diff < left / right > :
 thread '<unnamed>' panicked at 'Hello!', $DIR/simple.rs:LL:CC
 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<thread 'childthread' panicked at 'Hello, world!', $DIR/simple.rs:LL:CC
>  --> RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>   |
>   |
>LL |             libc::pthread_setname_np(libc::pthread_self(), name.as_ptr());
>   |
>   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
>   = note: backtrace:
>   = note: inside `std::sys::PLATFORM::thread::Thread::set_name` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>   = note: inside `std::sys::PLATFORM::thread::Thread::set_name` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>   = note: inside closure at RUSTLIB/std/src/thread/mod.rs:LL:CC
>   = note: inside `<[closure@std::thread::Builder::spawn_unchecked_<[closure@$DIR/simple.rs:LL:CC], ()>::{closure#1}] as std::ops::FnOnce<()>>::call_once - shim(vtable)` at RUSTLIB/core/src/ops/function.rs:LL:CC
>   = note: inside `<std::boxed::Box<std::boxed::Box<dyn std::ops::FnOnce()>> as std::ops::FnOnce<()>>::call_once` at RUSTLIB/alloc/src/boxed.rs:LL:CC
>   = note: inside `std::sys::PLATFORM::thread::Thread::new::thread_start` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>
>error: aborting due to previous error
---
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: unsupported operation: can't call foreign function: pthread_setname_np
  --> /checkout/library/std/src/sys/unix/thread.rs:137:13
   |
LL |             libc::pthread_setname_np(libc::pthread_self(), name.as_ptr());
   |
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: backtrace:
   = note: inside `std::sys::unix::thread::Thread::set_name` at /checkout/library/std/src/sys/unix/thread.rs:137:13
---
Diff < left / right > :
>error: unsupported operation: can't call foreign function: pthread_setname_np
>  --> RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>   |
>LL |             libc::pthread_setname_np(libc::pthread_self(), name.as_ptr());
>   |
>   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
>   = note: backtrace:
>   = note: inside `std::sys::PLATFORM::thread::Thread::set_name` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>   = note: inside `std::sys::PLATFORM::thread::Thread::set_name` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>   = note: inside closure at RUSTLIB/std/src/thread/mod.rs:LL:CC
>   = note: inside `<[closure@std::thread::Builder::spawn_unchecked_<[closure@$DIR/tls_lib_drop.rs:LL:CC], ()>::{closure#1}] as std::ops::FnOnce<()>>::call_once - shim(vtable)` at RUSTLIB/core/src/ops/function.rs:LL:CC
>   = note: inside `<std::boxed::Box<std::boxed::Box<dyn std::ops::FnOnce()>> as std::ops::FnOnce<()>>::call_once` at RUSTLIB/alloc/src/boxed.rs:LL:CC
>   = note: inside `std::sys::PLATFORM::thread::Thread::new::thread_start` at RUSTLIB/std/src/sys/PLATFORM/thread.rs:LL:CC
>
>error: aborting due to previous error
---
full stderr:
error: unsupported operation: can't call foreign function: pthread_setname_np
  --> /checkout/library/std/src/sys/unix/thread.rs:137:13
   |
LL |             libc::pthread_setname_np(libc::pthread_self(), name.as_ptr());
   |
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: backtrace:
   = note: inside `std::sys::unix::thread::Thread::set_name` at /checkout/library/std/src/sys/unix/thread.rs:137:13
---
+ NODE_PATH=/node-v14.4.0-linux-x64/lib/node_modules python3 ../x.py test src/test/rustdoc-gui --stage 2
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
⚠️ Installed version of browser-ui-test (`0.9.7:undefined`) is different than the one used in the CI (`0.9.7`)
You can install this version using `npm update browser-ui-test` or by using `npm install browser-ui-test@0.9.7`
    Finished release [optimized] target(s) in 0.17s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
---
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
⚠️ Installed version of browser-ui-test (`0.9.7:undefined`) is different than the one used in the CI (`0.9.7`)
You can install this version using `npm update browser-ui-test` or by using `npm install browser-ui-test@0.9.7`
    Finished release [optimized] target(s) in 0.19s
   Compiling test_docs v0.1.0 (/checkout/src/test/rustdoc-gui/src/test_docs)
 Documenting test_docs v0.1.0 (/checkout/src/test/rustdoc-gui/src/test_docs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.98s
---
................... (60/68)
........   (68/68)


/checkout/src/test/rustdoc-gui/toggle-docs.goml An exception occured: connect ECONNREFUSED 127.0.0.1:40827
== STACKTRACE ==
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:533:16)
    at runMicrotasks (<anonymous>)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
