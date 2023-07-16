plain
[00:19:02]    Compiling rustc-demangle v0.1.9
[00:19:03]    Compiling memmap v0.6.2
[00:19:03]    Compiling num_cpus v1.8.0
[00:19:10]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:19:18] error: unused import: `AddrSpaceOf`
[00:19:18]   --> librustc_codegen_llvm/mir/block.rs:26:19
[00:19:18]    |
[00:19:18] 26 | use type_::{Type, AddrSpaceOf, };
[00:19:18]    |
[00:19:18]    = note: `-D unused-imports` implied by `-D warnings`
[00:19:18] 
[00:19:18] error: unused import: `abi`
---
[00:19:18] 
[00:19:19] error[E0308]: match arms have incompatible types
[00:19:19]    --> librustc_codegen_llvm/abi.rs:589:27
[00:19:19]     |
[00:19:19] 589 |           let llreturn_ty = match self.ret.mode {
[00:19:19]     |  ___________________________^
[00:19:19] 590 | |             PassMode::Ignore => Type::void(cx),
[00:19:19] 591 | |             PassMode::Direct(_) | PassMode::Pair(..) => {
[00:19:19] 592 | |                 self.ret.layout.immediate_llvm_type(cx)
[00:19:19] 600 | |             }
[00:19:19] 601 | |         };
[00:19:19]     | |_________^ expected reference, found ()
[00:19:19]     |
[00:19:19]     |
[00:19:19]     = note: expected type `&llvm::ffi::::Type`
[00:19:19]                found type `()`
[00:19:19] note: match arm with an incompatible type
[00:19:19]    --> librustc_codegen_llvm/abi.rs:591:57
[00:19:19]     |
[00:19:19] 591 |               PassMode::Direct(_) | PassMode::Pair(..) => {
[00:19:19]     |  _________________________________________________________^
[00:19:19] 592 | |                 self.ret.layout.immediate_llvm_type(cx)
[00:19:19] 593 | |                   .as_flat_addr_space(cx);
[00:19:19]     | |_____________^
[00:19:19] 
[00:19:22] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:19:22]   --> librustc_codegen_llvm/mir/place.rs:70:13
[00:19:22]   --> librustc_codegen_llvm/mir/place.rs:70:13
[00:19:22]    |
[00:19:22] 70 |             consts::bitcast_elem(bx.cx, base_addr, Type::i8p(bx.cx)),
[00:19:22]    | 
[00:19:22]   ::: librustc_codegen_llvm/consts.rs:52:1
[00:19:22]    |
[00:19:22]    |
[00:19:22] 52 | pub fn bitcast_elem(val: &'ll Value, ty: &'ll Type) -> &'ll Value {
[00:19:22]    | ----------------------------------------------------------------- defined here
[00:19:22] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:19:22]   --> librustc_codegen_llvm/mir/place.rs:74:21
[00:19:22]    |
[00:19:22]    |
[00:19:22] 74 |         let llval = consts::bitcast_elem(bx.cx, llval, layout.llvm_type(bx.cx).ptr_to());
[00:19:22]    | 
[00:19:22]   ::: librustc_codegen_llvm/consts.rs:52:1
[00:19:22]    |
[00:19:22]    |
[00:19:22] 52 | pub fn bitcast_elem(val: &'ll Value, ty: &'ll Type) -> &'ll Value {
[00:19:22]    | ----------------------------------------------------------------- defined here
[00:19:22] 
[00:19:22] error[E0609]: no field `llvm` on type `mir::place::PlaceRef<'_, '_>`
[00:19:22]   --> librustc_codegen_llvm/mir/place.rs:92:42
[00:19:22]    |
[00:19:22] 92 |             llval: bx.flat_addr_cast(tmp.llvm),
[00:19:22]    |
[00:19:22]    |
[00:19:22]    = note: available fields are: `llval`, `llextra`, `layout`, `align`
[00:19:22] error: aborting due to 6 previous errors
[00:19:22] 
[00:19:22] Some errors occurred: E0061, E0308, E0609.
[00:19:22] For more information about an error, try `rustc --explain E0061`.
[00:19:22] For more information about an error, try `rustc --explain E0061`.
[00:19:22] error: Could not compile `rustc_codegen_llvm`.
[00:19:22] 
[00:19:22] Caused by:
[00:19:22]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=31f984e9d9483638 -C extra-filename=-31f984e9d9483638 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/deps --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps/libcc-11e0a9f073bf5c97.rlib --extern memmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps/libmemmap-4c58a61c767e77c2.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-9c2b6b79866500de.rlib --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-93a49a52ce463556.rlib --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-b4c8b22ef8c5e426.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/build/rustc_llvm-1f47acc9c60cd35e/out -L native=/usr/lib/llvm-5.0/lib` (exit code: 1)
[00:19:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
[00:19:22] expected success, got: exit code: 101
[00:19:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:19:22] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:19:22] travis_time:end:stage0-rustc_codegen_llvm:start=1536052710047939189,finish=1536052731621117607,duration=21573178418

[00:19:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:22] Build completed unsuccessfully in 0:14:45
[00:19:22] Makefile:28: recipe for target 'all' failed
[00:19:22] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a709e8c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:089bc2a8:start=1536052732370702187,finish=1536052732381308570,duration=10606383
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cf7375b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:010c6d13
travis_time:start:010c6d13
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cf028c8
$ dmesg | grep -i kill
