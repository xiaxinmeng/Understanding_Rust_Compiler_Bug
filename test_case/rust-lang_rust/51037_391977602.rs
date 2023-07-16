plain
[00:06:53]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:07:00]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:34]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:51]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:55] error[E0425]: cannot find value `UNUSED_LABELS` in this scope
[00:08:55]    --> librustc/lint/builtin.rs:328:13
[00:08:55] 328 |             UNUSED_LABELS,
[00:08:55]     |             ^^^^^^^^^^^^^ did you mean `UNUSED_MACROS`?
[00:08:55] 
[00:08:55] 
[00:08:55] error[E0425]: cannot find value `DUPLICATE_ASSOCIATED_TYPE_BINDINGS` in this scope
[00:08:55]    --> librustc/lint/builtin.rs:334:13
[00:08:55] 334 |             DUPLICATE_ASSOCIATED_TYPE_BINDINGS,
[00:08:55]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:08:55] 
[00:09:12] error[E0308]: mismatched types
[00:09:12] error[E0308]: mismatched types
[00:09:12]     --> librustc/middle/resolve_lifetime.rs:1307:17
[00:09:12]      |
[00:09:12] 1307 |                 None => {
[00:09:12]      |                 ^^^^ expected enum `middle::resolve_lifetime::LifetimeUseSet`, found enum `std::option::Option`
[00:09:12]      |
[00:09:12]      = note: expected type `middle::resolve_lifetime::LifetimeUseSet<'_>`
[00:09:12]                 found type `std::option::Option<_>`
[00:09:12] error[E0308]: mismatched types
[00:09:12]     --> librustc/middle/resolve_lifetime.rs:1308:65
[00:09:12]      |
[00:09:12]      |
[00:09:12] 1308 |                     let node_id = self.tcx.hir.as_local_node_id(def_id).unwrap();
[00:09:12]      |                                                                 |
[00:09:12]      |                                                                 |
[00:09:12]      |                                                                 expected struct `hir::def_id::DefId`, found reference
[00:09:12]      |                                                                 help: consider dereferencing the borrow: `*def_id`
[00:09:12]      |
[00:09:12]      = note: expected type `hir::def_id::DefId`
[00:09:12]                 found type `&hir::def_id::DefId`
[00:09:23] error: aborting due to 4 previous errors
[00:09:23] 
[00:09:23] Some errors occurred: E0308, E0425.
[00:09:23] For more information about an error, try `rustc --explain E0308`.
[00:09:23] For more information about an error, try `rustc --explain E0308`.
[00:09:23] error: Could not compile `rustc`.
[00:09:23] 
[00:09:23] Caused by:
[00:09:23]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=ccde15cd50067d0a -C extra-filename=-ccde15cd50067d0a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-50bb024c677a86d3.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-97bf376e0a007ec9.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-79aaef6d504f0382.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-eaacc24e35645dbd.rlib --extern graphviz=/checkout/obj/build/x8:09:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:23] Build completed unsuccessfully in 0:03:23
[00:09:23] Makefile:28: recipe for target 'all' failed
[00:09:23] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0025f9e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
