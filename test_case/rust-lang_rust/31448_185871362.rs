
---- [run-pass] run-pass-fulldeps/mir-pass.rs stdout ----

error: auxiliary build of "/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs" failed to compile: 
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs -L x86_64-unknown-linux-gnu/test/run-pass-fulldeps/ --target=x86_64-unknown-linux-gnu --crate-type=dylib -L x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mir-pass.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic --out-dir x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mir-pass.stage2-x86_64-unknown-linux-gnu.run-pass.libaux --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:21:35: 21:47 error: unresolved import `rustc::mir::transform::MirBlockPass`. There is no `MirBlockPass` in `rustc::mir::transform` [E0432]
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:21 use rustc::mir::transform::{self, MirBlockPass};
                                                                                                            ^~~~~~~~~~~~
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:21:35: 21:47 help: run `rustc --explain E0432` to see a detailed explanation
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:34:5: 36:6 error: method `priority` is not a member of trait `transform::Pass` [E0407]
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:34     fn priority(&self) -> usize {
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:35         1000
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:36     }
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:34:5: 36:6 help: run `rustc --explain E0407` to see a detailed explanation
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:39:6: 39:18 error: `MirBlockPass` is not a trait [E0404]
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:39 impl MirBlockPass for Pass {
                                                                               ^~~~~~~~~~~~
/home/travis/build/rust-lang/rust/src/test/auxiliary/dummy_mir_pass.rs:39:6: 39:18 help: run `rustc --explain E0404` to see a detailed explanation
error: cannot continue compilation due to previous error

------------------------------------------

thread '[run-pass] run-pass-fulldeps/mir-pass.rs' panicked at 'explicit panic', /home/travis/build/rust-lang/rust/src/compiletest/runtest.rs:1529
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass-fulldeps/mir-pass.rs
