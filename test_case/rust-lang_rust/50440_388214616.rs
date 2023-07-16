plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/18/61/4e0f977cfe063188d73622a91cab8b8b409b662f422303fc687f362d941f/awscli-1.15.18-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▉                               | 30kB 1.9MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
[00:05:37]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:45]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:15]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:31]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:44] error[E0596]: cannot borrow immutable local variable `def_ids` as mutable
[00:08:44]     --> librustc/middle/resolve_lifetime.rs:1320:9
[00:08:44]      |
[00:08:44] 1309 |         let def_ids: Vec<_> = defined_by.values()
[00:08:44]      |             ------- consider changing this to `mut def_ids`
[00:08:44] ...
[00:08:44] 1320 |         def_ids.sort_by_key(|&def_id| self.tcx.def_path_hash(def_id));
[00:08:44]      |         ^^^^^^^ cannot borrow mutably
[00:08:48] error: aborting due to previous error
[00:08:48] 
[00:08:48] For more information about this error, try `rustc --explain E0596`.
[00:08:48] error: Could not compile `rustc`.
[00:08:48] error: Could not compile `rustc`.
[00:08:48] 
[00:08:48] Caused by:
[00:08:48]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=dec7324c263ceb0f -C extra-filename=-dec7324c263ceb0f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-33787dcdac3a4dd2.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b7c66a9cab3ff5a6.rlib --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-24d411fab3d9dcf5.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-21ce4bd19908f0cc.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-d0ff1c27bea11ea7.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-fbe97ade1df6b749.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-8d928be2ff984c7f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-16776be762f4e8c2.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-dd375c0772cf1a0d.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-566a8d95e6a18781.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-ee923a086d887011.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-c0908caf79d2e3f2.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ffd9dcc5ce13143f/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-41d448831c9d08f1/out` (exit code: 101)
[00:08:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:48] expected success, got: exit code: 101
[00:08:48] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:48] travis_fold:end:stage0-rustc

[00:08:48] travis_time:end:stage0-rustc:start=1525994255724930212,finish=1525994486492290376,duration=230767360164


[00:08:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:48] Build completed unsuccessfully in 0:04:04
[00:08:48] Makefile:28: recipe for target 'all' failed
[00:08:48] make: *** [all] Error 1
47168 ./src/test
46812 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
46352 ./src/llvm/test/MC
44444 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
