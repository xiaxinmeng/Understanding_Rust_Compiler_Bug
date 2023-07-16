
$ ./x.py build src/tools/{rls,cargo}
...
duplicate artfacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

  num-traits 0.2.1 (registry+https://github.com/rust-lang/crates.io-index)
    `cargo` enabled features [] at "libnum_traits-58f093f238cd6fd3.rlib"
    `rls` enabled features ["default", "std"] at "libnum_traits-3b34cffa1e11f667.rlib"
  serde_json 1.0.10 (registry+https://github.com/rust-lang/crates.io-index)
    `cargo` enabled features ["default"] at "libserde_json-f3e37e0dddd3eac2.rlib"
    `rls` enabled features ["default"] at "libserde_json-14e1634b89a3f35d.rlib"

thread 'main' panicked at 'tools should not compile multiple copies of the same crate', bootstrap/tool.rs:168:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /home/alex/code/rust4/build/bootstrap/debug/bootstrap build src/tools/rls src/tools/cargo
Build completed unsuccessfully in 0:00:03
