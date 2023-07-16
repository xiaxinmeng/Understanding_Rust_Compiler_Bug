
Building stage2 tool miri (x86_64-unknown-linux-gnu)
[00:55:21]  Downloading log_settings v0.1.1
[00:55:21]  Downloading byteorder v1.1.0
[00:55:21]  Downloading libc v0.2.30
[00:55:22]  Downloading backtrace-sys v0.1.12
[00:55:22]  Downloading gcc v0.3.53
[00:55:22]    Compiling log v0.3.8
[00:55:22]    Compiling byteorder v1.1.0
[00:55:22]    Compiling cfg-if v0.1.2
[00:55:22]    Compiling gcc v0.3.53
[00:55:22]    Compiling miri v0.1.0 (file:///checkout/src/tools/miri)
[00:55:23]    Compiling rustc-demangle v0.1.5
[00:55:23]    Compiling libc v0.2.30
[00:55:24]    Compiling utf8-ranges v1.0.0
[00:55:24]    Compiling void v1.0.2
[00:55:25]    Compiling unreachable v1.0.0
[00:55:25]    Compiling log_settings v0.1.1
[00:55:25]    Compiling thread_local v0.3.4
[00:55:25]    Compiling memchr v1.0.1
[00:55:26]    Compiling aho-corasick v0.6.3
[00:55:26]    Compiling regex v0.2.2
[00:55:30]    Compiling backtrace-sys v0.1.12
[00:55:38]    Compiling backtrace v0.3.3
[00:55:54]    Compiling rustc_miri v0.1.0 (file:///checkout/src/tools/miri/src/librustc_mir)
[00:55:54]    Compiling env_logger v0.4.3
[00:55:55] error[E0425]: cannot find function `get_vtable_methods` in module `rustc::traits`
[00:55:55]   --> /checkout/src/tools/miri/src/librustc_mir/interpret/traits.rs:57:40
[00:55:55]    |
[00:55:55] 57 |         let methods = ::rustc::traits::get_vtable_methods(self.tcx, trait_ref);
[00:55:55]    |                                        ^^^^^^^^^^^^^^^^^^ did you mean `vtable_methods`?
[00:55:55] 
[00:55:55] error[E0425]: cannot find function `get_vtable_methods` in module `rustc::traits`
[00:55:55]   --> /checkout/src/tools/miri/src/librustc_mir/interpret/traits.rs:73:45
[00:55:55]    |
[00:55:55] 73 |         for (i, method) in ::rustc::traits::get_vtable_methods(self.tcx, trait_ref).enumerate() {
[00:55:55]    |                                             ^^^^^^^^^^^^^^^^^^ did you mean `vtable_methods`?
[00:55:55] 
[00:55:56] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[00:55:56]     --> /checkout/src/tools/miri/src/librustc_mir/interpret/eval_context.rs:2419:45
[00:55:56]      |
[00:55:56] 2419 |     let vtbl = tcx.trans_fulfill_obligation(DUMMY_SP, ty::Binder(trait_ref));
[00:55:56]      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 parameter
[00:55:56] 
[00:55:57] error: aborting due to 3 previous errors
[00:55:57] 
[00:55:57] error: Could not compile `rustc_miri`.
[00:55:57] 
[00:55:57] To learn more, run the command again with --verbose.
[00:55:57] This failure is expected (see `src/tools/toolstate.toml`)
[00:55:57] thread 'main' panicked at 'failed to copy `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/miri` to `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri`: the source path is not an existing regular file', /checkout/src/bootstrap/util.rs:44:8
[00:55:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
