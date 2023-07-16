plain
Receiving objects: 100% (237/237), 22.31 KiB | 22.31 MiB/s, done.
---
Resolving deltas: 100% (209/209), completed with 63 local objects.
---
[00:00:48] configure: rust.quiet-tests     := True
---
[00:24:39] error[E0658]: non-reference pattern used to match a reference (see issue #42640)
[00:24:39]    --> librustc_trans/back/symbol_export.rs:200:12
[00:24:39]     |
[00:24:39] 200 |     if let Some(id) = tcx.sess.derive_registrar_fn.get() {
[00:24:39]     |            ^^^^^^^^ help: consider using a reference: `&Some(id)`
[00:24:39]     |
[00:24:39]     = help: add #![feature(match_default_bindings)] to the crate attributes to enable
[00:24:39]
[00:24:39] error[E0308]: mismatched types
[00:24:39]    --> librustc_trans/back/symbol_export.rs:201:43
[00:24:39]     |
[00:24:39] 201 |         let def_id = tcx.hir.local_def_id(id);
[00:24:39]     |                                           ^^
[00:24:39]     |                                           |
[00:24:39]     |                                           expected struct `syntax::ast::NodeId`, found reference
[00:24:39]     |                                           help: consider dereferencing the borrow: `*id`
[00:24:39]     |
[00:24:39]     = note: expected type `syntax::ast::NodeId`
[00:24:39]                found type `&syntax::ast::NodeId`
[00:24:39]
[00:24:39] error[E0658]: non-reference pattern used to match a reference (see issue #42640)
[00:24:39]    --> librustc_trans/back/symbol_export.rs:205:12
[00:24:39]     |
[00:24:39] 205 |     if let Some(id) = tcx.sess.plugin_registrar_fn.get() {
[00:24:39]     |            ^^^^^^^^ help: consider using a reference: `&Some(id)`
[00:24:39]     |
[00:24:39]     = help: add #![feature(match_default_bindings)] to the crate attributes to enable
[00:24:39]
[00:24:39] error[E0308]: mismatched types
[00:24:39]    --> librustc_trans/back/symbol_export.rs:206:43
[00:24:39]     |
[00:24:39] 206 |         let def_id = tcx.hir.local_def_id(id);
[00:24:39]     |                                           ^^
[00:24:39]     |                                           |
[00:24:39]     |                                           expected struct `syntax::ast::NodeId`, found reference
[00:24:39]     |                                           help: consider dereferencing the borrow: `*id`
[00:24:39]     |
[00:24:39]     = note: expected type `syntax::ast::NodeId`
[00:24:39]                found type `&syntax::ast::NodeId`
[00:24:39]
[00:24:40] error[E0308]: mismatched types
[00:24:40]    --> librustc_trans/base.rs:520:14
[00:24:40]     |
[00:24:40] 520 |         Some((id, span)) => {
[00:24:40]     |              ^^^^^^^^^^ expected a tuple with 3 elements, found one with 2 elements
[00:24:40]     |
[00:24:40]     = note: expected type `(syntax::ast::NodeId, syntax_pos::Span, rustc::session::config::EntryFnType)`
[00:24:40]                found type `(_, _)`
[00:24:40]
[00:24:40] error[E0609]: no field `entry_type` on type `&rustc::session::Session`
[00:24:40]    --> librustc_trans/base.rs:536:24
[00:24:40]     |
[00:24:40] 536 |     let et = cx.sess().entry_type.get().map(|&(_, _, et)| et);
---
[00:24:42]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_trans librustc_trans/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_back" -C metadata=73a666260136fd4b -C extra-filename=-73a666260136fd4b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-3c385cb05f9c08fa.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-3a5b6600c0885885.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-50848c58fc469522.so --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-179e4ed8bcb4d938.rlib --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-078085824a3af3a6.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7437cee1018df6d3.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-9c0e4dc84dafd637.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-02ca437d4346e274.so --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-8a1b2d4c707edb2f.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-eaa7f15360ed28d2.so --extern rustc_trans_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trans_utils-00de41b71dadf4c8.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-5472f3f0fa044374.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-7eb604ad0d7f2749.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-0249ed74490015f9.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-9784fba5d291c443.so --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-0fd2999d51f284b0.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-728fcbc86125f341.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-f908eec6ea19d085.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e8abef14b8b443aa.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-505a23f516928b0b.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b23276248c684a7d.rlib --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-25cbf88f884a94e4.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/mig' || true)
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0e04ae7c:start=1522647641147684696,finish=1522647641153253265,duration=5568569
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:032e5a7e
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:032e5a7e:start=1522647641158135560,finish=1522647641163704935,duration=5569375
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0efa8401
$ dmesg | grep -i kill
[   11.340202] init: failsafe main process (1093) killed by TERM signal
