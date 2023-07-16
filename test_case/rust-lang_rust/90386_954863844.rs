plain
    Finished release [optimized] target(s) in 3m 05s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: failed to run `rustc` to learn about target-specific information
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 101)
  --- stderr
  thread 'main' panicked at 'No option 'assert-incr-comp' defined', /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:799:21

  error: internal compiler error: unexpected panic

  note: the compiler unexpectedly panicked. this is a bug.
  note: the compiler unexpectedly panicked. this is a bug.

  note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

  note: rustc 1.58.0-nightly (6df22b13a 2021-10-29) running on x86_64-unknown-linux-gnu

Build completed unsuccessfully in 0:04:00
  note: compiler flags: -Z symbol-mangling-version=legacy -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z force-unstable-if-unmarked -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro
  query stack during panic:
  end of query stack
