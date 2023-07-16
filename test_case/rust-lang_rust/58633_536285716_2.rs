
       Fresh cmake v0.1.38
     Running `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name core src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=72dd04d070b09a5d -C extra-filename=-72dd04d070b09a5d --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Zbinary-dep-depinfo`
rustc command: "LD_LIBRARY_PATH"="/home/user/.cargo/lib:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps:/home/user/build/2nonpkgs/rust.stuff/rust/rust.installed.dir/lib" "/home/user/.cargo/bin/rustc" "--edition=2018" "--crate-name" "core" "src/libcore/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=2" "-C" "codegen-units=1" "-C" "debuginfo=2" "-C" "metadata=72dd04d070b09a5d" "-C" "extra-filename=-72dd04d070b09a5d" "--out-dir" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/incremental" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "-Zexternal-macro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Dwarnings" "-Cprefer-dynamic" "-Zbinary-dep-depinfo" "--sysroot" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-C" "debug-assertions=n" "-Z" "force-unstable-if-unmarked"
sysroot: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot"
libdir: "/home/user/.cargo/lib"
error: unused attribute
   --> src/libcore/convert.rs:564:1
    |
564 | / #[rustc_reservation_impl="permitting this impl would forbid us from adding \
565 | | `impl<T> From<!> for T` later; see rust-lang/rust#64715 for details"]
    | |_____________________________________________________________________^
    |
    = note: `-D unused-attributes` implied by `-D warnings`

error: unused attribute
  --> src/libcore/slice/mod.rs:66:5
   |
66 |     #[allow_internal_unstable(const_fn_union)]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused attribute
    --> src/libcore/str/mod.rs:2170:5
     |
2170 |     #[allow_internal_unstable(const_fn_union)]
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 3 previous errors

[RUSTC-TIMING] core test:false 16.681
error: could not compile `core`.

Caused by:
  process didn't exit successfully: `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name core src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=72dd04d070b09a5d -C extra-filename=-72dd04d070b09a5d --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Zbinary-dep-depinfo` (exit code: 1)
command did not execute successfully: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 900, in main
    bootstrap(help_triggered)
  File "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 886, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 141, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv -j 4

real	0m22.882s
user	0m22.963s
sys	0m2.961s

