plain
[00:07:21]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:42]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:11]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:11]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:11] error: expected one of `=>`, `if`, or `|`, found `Some`
[00:14:11]    --> librustc_mir/borrow_check/error_reporting.rs:646:13
[00:14:11]     |
[00:14:11] 645 |             }))), ..})
[00:14:11]     |                       - expected one of `=>`, `if`, or `|` here
[00:14:11] 646 |             Some(LocalDecl { is_user_variable: None, .. })
[00:14:11]     |             ^^^^ unexpected token
[00:14:11] 
[00:14:13] error: unused imports: `BindingForm`, `ClearCrossCrate`
[00:14:13]   --> librustc_mir/borrow_check/error_reporting.rs:13:18
[00:14:13]    |
[00:14:13] 13 | use rustc::mir::{BindingForm, BorrowKind, ClearCrossCrate, Field, Local};
[00:14:13]    |
[00:14:13]    = note: `-D unused-imports` implied by `-D warnings`
[00:14:13] 
[00:14:13] 
[00:14:13] error: unused import: `LocalDecl`
[00:14:13]   --> librustc_mir/borrow_check/error_reporting.rs:14:18
[00:14:13]    |
[00:14:13] 14 | use rustc::mir::{LocalDecl, LocalKind, Location, Operand, Place};
[00:14:13] 
[00:14:13] 
[00:14:13] error: unused import: `rustc::mir::VarBindingForm`
[00:14:13]   --> librustc_mir/borrow_check/error_reporting.rs:16:5
[00:14:13]    |
[00:14:13] 16 | use rustc::mir::VarBindingForm;
[00:14:13] 
[00:14:15] error: unreachable statement
[00:14:15]    --> librustc_mir/borrow_check/error_reporting.rs:651:9
[00:14:15]     |
[00:14:15]     |
[00:14:15] 651 | /         let mut err = self.tcx.cannot_reassign_immutable(
[00:14:15] 652 | |             span,
[00:14:15] 653 | |             place_description.as_ref().map(AsRef::as_ref).unwrap_or("_"),
[00:14:15] 654 | |             from_arg,
[00:14:15] 655 | |             Origin::Mir,
[00:14:15] 656 | |         );
[00:14:15]     |
[00:14:15]     = note: `-D unreachable-code` implied by `-D warnings`
[00:14:15] 
[00:14:24] error: aborting due to 5 previous errors
[00:14:24] error: aborting due to 5 previous errors
[00:14:24] 
[00:14:24] error: Could not compile `rustc_mir`.
[00:14:24] 
[00:14:24] Caused by:
[00:14:24]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=cfff5c37c787835a -C extra-filename=-cfff5c37c787835a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f0ddda7cc3e4e4d5.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-fcb7f2ed7949c0c2.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b9c6a6d70ded088a.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c73ed1030caf17c3.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:16:12] error: build failed
[00:16:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:12] expected success, got: exit code: 101
[00:16:12] expected success, got: exit code: 101
[00:16:12] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:12] travis_fold:end:stage0-rustc

[00:16:12] travis_time:end:stage0-rustc:start=1530314204991788950,finish=1530314881092018956,duration=676100230006

