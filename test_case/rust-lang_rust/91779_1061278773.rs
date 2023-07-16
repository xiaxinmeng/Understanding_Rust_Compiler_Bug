plain
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 229 tests
..........i.ii.....ii............................................................................... 100/229
.................i......F..........F.iiiiiii......i...................iii....F...................... 200/229
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/mixing-formats stdout ----
---- [run-make] run-make-fulldeps/mixing-formats stdout ----

error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# Building just baz
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats  --crate-type=rlib  foo.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats  --crate-type=dylib bar1.rs -C prefer-dynamic
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats  --crate-type=dylib,rlib baz.rs -C prefer-dynamic
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats  --crate-type=bin baz.rs
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats/*
--- stderr -------------------------------
warning: function is never used: `main`
warning: function is never used: `main`
 --> baz.rs:3:4
3 | fn main() {}
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

rm: cannot remove '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats/visualizers': Is a directory
make: *** [Makefile:21: all] Error 1


---- [run-make] run-make-fulldeps/output-type-permutations stdout ----


error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations  foo.rs --crate-type=rlib,dylib,staticlib
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations/libbar*.rlib
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations/libbar*.so
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations/libbar.a
rm -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations/{lib,}bar.{dll.exp,dll.lib,pdb,dll.a}
# Check that /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations is empty.
[ "$(ls -1 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/output-type-permutations/output-type-permutations | wc -l)" -eq "0" ]
--- stderr -------------------------------
warning: function is never used: `main`
 --> foo.rs:3:4
  |
  |
3 | fn main() {}
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

make: *** [Makefile:10: all] Error 1



---- [run-make] run-make-fulldeps/separate-link stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
echo 'fn main(){}' | LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link  -Z no-link -
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link  -Z link-only /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/separate-link/separate-link/rust_out.rlink
--- stderr -------------------------------
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_session/src/session.rs:309:41
   0:     0x7fa0a86385dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5cbf65cc1f845500
   1:     0x7fa0a869fa2f - core::fmt::write::h530533a18da863d3
   2:     0x7fa0a8627b41 - std::io::Write::write_fmt::hc481558018ceb992
   3:     0x7fa0a86383fb - std::sys_common::backtrace::print::hebd22fa5f6c7e811
   3:     0x7fa0a86383fb - std::sys_common::backtrace::print::hebd22fa5f6c7e811
   4:     0x7fa0a863bb34 - std::panicking::default_hook::{{closure}}::h1d6d0daf60ec0f6a
   5:     0x7fa0a863b6ed - std::panicking::default_hook::hdcf680879ba78562
   6:     0x7fa0a90b9a11 - rustc_driver[41e592c6437bee78]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa0a863c21a - std::panicking::rust_panic_with_hook::hac8cf6908e37784e
   8:     0x7fa0a863c009 - std::panicking::begin_panic_handler::{{closure}}::h96a38dc2289a2b78
   9:     0x7fa0a8638af4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4137627fa1a71ea7
  10:     0x7fa0a863bd29 - rust_begin_unwind
  11:     0x7fa0a85e9c03 - core::panicking::panic_fmt::h635416521b51c09a
  12:     0x7fa0a85e9acd - core::panicking::panic::hb89ccf0c1bb24236
  13:     0x7fa0ab899919 - <rustc_session[1f80207608e64a10]::session::Session>::debugger_visualizers
  14:     0x7fa0aab02456 - rustc_codegen_ssa[e6d777066f6db12d]::back::link::add_order_independent_options
  15:     0x7fa0a93e434e - rustc_codegen_ssa[e6d777066f6db12d]::back::link::link_natively::<rustc_codegen_llvm[e4074b208d0b3ef0]::back::archive::LlvmArchiveBuilder>
  16:     0x7fa0a93e3151 - rustc_codegen_ssa[e6d777066f6db12d]::back::link::link_binary::<rustc_codegen_llvm[e4074b208d0b3ef0]::back::archive::LlvmArchiveBuilder>
  17:     0x7fa0a93bf305 - <rustc_codegen_llvm[e4074b208d0b3ef0]::LlvmCodegenBackend as rustc_codegen_ssa[e6d777066f6db12d]::traits::backend::CodegenBackend>::link
  18:     0x7fa0a90b5719 - rustc_driver[41e592c6437bee78]::try_process_rlink
  19:     0x7fa0a90a4099 - rustc_span[41af263066920550]::with_source_map::<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, rustc_interface[a702127163483711]::interface::create_compiler_and_run<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, rustc_driver[41e592c6437bee78]::run_compiler::{closure#1}>::{closure#1}>
  20:     0x7fa0a9119693 - <scoped_tls[ba026f876effcec]::ScopedKey<rustc_span[41af263066920550]::SessionGlobals>>::set::<rustc_interface[a702127163483711]::interface::run_compiler<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, rustc_driver[41e592c6437bee78]::run_compiler::{closure#1}>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>
  21:     0x7fa0a90e0409 - std[b01119826418e50a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a702127163483711]::util::run_in_thread_pool_with_globals<rustc_interface[a702127163483711]::interface::run_compiler<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, rustc_driver[41e592c6437bee78]::run_compiler::{closure#1}>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>
  22:     0x7fa0a91344f1 - std[b01119826418e50a]::panicking::try::<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, core[93ef6fd691ff4976]::panic::unwind_safe::AssertUnwindSafe<<std[b01119826418e50a]::thread::Builder>::spawn_unchecked_<rustc_interface[a702127163483711]::util::run_in_thread_pool_with_globals<rustc_interface[a702127163483711]::interface::run_compiler<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, rustc_driver[41e592c6437bee78]::run_compiler::{closure#1}>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7fa0a90e1f22 - <<std[b01119826418e50a]::thread::Builder>::spawn_unchecked_<rustc_interface[a702127163483711]::util::run_in_thread_pool_with_globals<rustc_interface[a702127163483711]::interface::run_compiler<core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>, rustc_driver[41e592c6437bee78]::run_compiler::{closure#1}>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>::{closure#0}, core[93ef6fd691ff4976]::result::Result<(), rustc_errors[50e44bb430c6696]::ErrorGuaranteed>>::{closure#1} as core[93ef6fd691ff4976]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7fa0a8647a23 - std::sys::unix::thread::Thread::new::thread_start::he71196c690af9061
  25:     0x7fa0a29b4609 - start_thread
  26:     0x7fa0a84ab293 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (565f26b09 2022-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z link-only
query stack during panic:
end of query stack
end of query stack
make: *** [Makefile:5: all] Error 101



failures:
