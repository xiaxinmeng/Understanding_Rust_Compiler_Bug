
   Compiling fuchsia-zircon v0.3.1 (file:///b/s/w/ir/kitchen-workdir/out/debug-x86-64/gen/garnet/public/rust/crates/fuchsia-zircon/fuchsia-zircon.rust)
     Running `/b/s/w/ir/kitchen-workdir/buildtools/linux-x64/rust/bin/rustc --crate-name fuchsia_zircon /b/s/w/ir/kitchen-workdir/out/debug-x86-64/gen/garnet/public/rust/crates/fuchsia-zircon/fuchsia-zircon.rust/../../../../../../../../../garnet/public/rust/crates/fuchsia-zircon/src/lib.rs --crate-type lib --emit=dep-info,metadata -C debuginfo=2 -C metadata=6b4de983d09d3465 -C extra-filename=-6b4de983d09d3465 --out-dir /b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/x86_64-unknown-fuchsia/debug/deps --target x86_64-unknown-fuchsia -C linker=/b/s/w/ir/kitchen-workdir/buildtools/linux-x64/clang/bin/clang -L dependency=/b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/x86_64-unknown-fuchsia/debug/deps -L dependency=/b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/debug/deps --extern bitflags=/b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/x86_64-unknown-fuchsia/debug/deps/libbitflags-d4fd6e1326a636c6.rmeta --extern fuchsia_zircon_sys=/b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/x86_64-unknown-fuchsia/debug/deps/libfuchsia_zircon_sys-6292aa11792b1921.rmeta -Clink-arg=--target=x86_64-unknown-fuchsia -Clink-arg=--sysroot=/b/s/w/ir/kitchen-workdir/out/build-zircon/build-zircon-pc-x86-64/sysroot`
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.23.0-nightly (6e2977499 2017-11-08) running on x86_64-unknown-linux-gnu
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'index out of bounds: the len is 729088 but the index is 754414', /checkout/src/libserialize/leb128.rs:59:20
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:69
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:58
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: <rustc_metadata::decoder::DecodeContext<'doc, 'tcx> as serialize::serialize::Decoder>::read_str
  11: <syntax_pos::symbol::Symbol as serialize::serialize::Decodable>::decode
  12: serialize::serialize::Decoder::read_struct
  13: rustc_metadata::decoder::<impl rustc_metadata::cstore::MetadataBlob>::get_root
  14: _ZN14rustc_metadata7locator7Context11extract_one17h7db1053efdbe4addE.llvm.E31DAB83
  15: rustc_metadata::locator::Context::maybe_load_library_crate
  16: _ZN14rustc_metadata7creader11CrateLoader4load17h19282c12bb9daf18E.llvm.301FDDB9
  17: _ZN14rustc_metadata7creader11CrateLoader13resolve_crate17h8a0c06f5f96b65c6E.llvm.301FDDB9
  18: _ZN86_$LT$core..iter..Chain$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hc7b85c83351c6246E.llvm.A1464E27
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  20: _ZN14rustc_metadata7creader11CrateLoader13resolve_crate17h8a0c06f5f96b65c6E.llvm.301FDDB9
  21: _ZN86_$LT$core..iter..Chain$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hc7b85c83351c6246E.llvm.A1464E27
  22: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  23: _ZN14rustc_metadata7creader11CrateLoader13resolve_crate17h8a0c06f5f96b65c6E.llvm.301FDDB9
  24: <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateLoader>::process_item
  25: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::build_reduced_graph_for_item
  26: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item
  27: syntax::visit::walk_item
  28: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item
  29: syntax::ext::expand::Expansion::visit_with
  30: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::visit_expansion
  31: _ZN6syntax3ext6expand13MacroExpander19collect_invocations17h9d5196537df48be7E.llvm.11F23753
  32: _ZN6syntax3ext6expand13MacroExpander6expand17he1792b62f0a2bbe9E.llvm.11F23753
  33: syntax::ext::expand::MacroExpander::expand_crate
  34: _ZN12rustc_driver6driver28phase_2_configure_and_expand28_$u7b$$u7b$closure$u7d$$u7d$17h146c38c9c1dd7f58E.llvm.6ACA7ED9
  35: rustc::util::common::time
  36: rustc_driver::driver::compile_input
  37: rustc_driver::run_compiler
error: Could not compile `slab`.
Caused by:
  process didn't exit successfully: `/b/s/w/ir/kitchen-workdir/buildtools/linux-x64/rust/bin/rustc --crate-name slab /b/s/w/ir/kitchen-workdir/third_party/rust-crates/vendor/slab-0.4.0/src/lib.rs --crate-type lib --emit=dep-info,metadata -C debuginfo=2 -C metadata=2757223a724c06ee -C extra-filename=-2757223a724c06ee --out-dir /b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/x86_64-unknown-fuchsia/debug/deps --target x86_64-unknown-fuchsia -C linker=/b/s/w/ir/kitchen-workdir/buildtools/linux-x64/clang/bin/clang -L dependency=/b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/x86_64-unknown-fuchsia/debug/deps -L dependency=/b/s/w/ir/kitchen-workdir/out/debug-x86-64/obj/garnet/public/lib/device_settings/fidl/fidl_rust_library.rust/debug/deps --cap-lints allow -Clink-arg=--target=x86_64-unknown-fuchsia -Clink-arg=--sysroot=/b/s/w/ir/kitchen-workdir/out/build-zircon/build-zircon-pc-x86-64/sysroot` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
error: build failed
