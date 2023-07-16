

---- [run-make] run-make-fulldeps/c-link-to-rust-va-list-fn stdout ----



error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[2]: Entering directory '/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/run-make-fulldeps/c-link-to-rust-va-list-fn'
LD_LIBRARY_PATH="/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-make-fulldeps/c-link-to-rust-va-list-fn/c-link-to-rust-va-list-fn:/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/lib:/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage0-bootstrap-tools/s390x-unknown-linux-gnu/release/deps:/usr/lib:" '/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc' --out-dir /<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-make-fulldeps/c-link-to-rust-va-list-fn/c-link-to-rust-va-list-fn -L /<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/run-make-fulldeps/c-link-to-rust-va-list-fn/c-link-to-rust-va-list-fn  checkrust.rs
make[2]: Leaving directory '/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/run-make-fulldeps/c-link-to-rust-va-list-fn'

------------------------------------------
stderr:
------------------------------------------
LLVM ERROR: Cannot select: 0x3ff88038650: i64,ch = vaarg 0x3ff88007178, 0x3ff88038448, SrcValue:ch<0x14fb194f0>, TargetConstant:i32<8>
  0x3ff88038448: i64,ch = CopyFromReg 0x3ff88007178, Register:i64 %1
    0x3ff880383e0: i64 = Register %1
  0x3ff880385e8: i32 = TargetConstant<8>
In function: _ZN4core3ffi6VaList3arg17h1165c011967c3d25E
Segmentation fault
make[2]: *** [Makefile:4: all] Error 139

------------------------------------------

thread '[run-make] run-make-fulldeps/c-link-to-rust-va-list-fn' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-make] run-make-fulldeps/c-link-to-rust-va-list-fn

test result: FAILED. 191 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
