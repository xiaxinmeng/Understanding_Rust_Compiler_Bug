plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/static-unwinding/static-unwinding:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/static-unwinding/static-unwinding -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/static-unwinding/static-unwinding  lib.rs
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/static-unwinding/static-unwinding:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/static-unwinding/static-unwinding -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/static-unwinding/static-unwinding  main.rs
--- stderr -------------------------------
--- stderr -------------------------------
warning: static variable `statik` should have an upper case name
  |
  |
3 | pub static mut statik: isize = 0;
  |                ^^^^^^ help: convert the identifier to upper case: `STATIK`
  = note: `#[warn(non_upper_case_globals)]` on by default

warning: 1 warning emitted


warning: static variable `statik` should have an upper case name
 --> main.rs:5:12
  |
5 | static mut statik: isize = 0;
  |            ^^^^^^ help: convert the identifier to upper case: `STATIK`
  = note: `#[warn(non_upper_case_globals)]` on by default


rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/GlobalISel/RegisterBankInfo.cpp:600: bool llvm::RegisterBankInfo::InstructionMapping::verify(const llvm::MachineInstr&) const: Assertion `NumOperands == (isCopyLike(MI) ? 1 : MI.getNumOperands()) && "NumOperands must match, see constructor"' failed.
Aborted (core dumped)
make: *** [Makefile:5: all] Error 134



failures:
