plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/12/92bd3655f436aa009688e7ccb8b7581554fb64278d111f5845e79da8e618/awscli-1.16.14-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:22:50]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:22:58] error: found removed `do catch` syntax
[00:22:58]     --> libsyntax/parse/parser.rs:2084:40
[00:22:58]      |
[00:22:58] 2084 |                 let args: PResult<_> = do catch {
[00:22:58]      |
[00:22:58]      = help: Following RFC #2388, the new non-placeholder syntax is `try`
[00:22:58] 
[00:22:58] 
[00:22:59] error: unused import: `AngleBracketedArgs`
[00:22:59]    |
[00:22:59]    |
[00:22:59] 12 | use ast::{AngleBracketedArgs, ParenthesisedArgs, AttrStyle, BareFnTy};
[00:22:59]    |
[00:22:59]    = note: `-D unused-imports` implied by `-D warnings`
[00:22:59] 
[00:23:06] error: aborting due to 2 previous errors
[00:23:06] error: aborting due to 2 previous errors
[00:23:06] 
[00:23:06] error: Could not compile `syntax`.
[00:23:06] 
[00:23:06] Caused by:
[00:23:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=9e932be7f344cce6 -C extra-filename=-9e932be7f344cce6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-fb1b999a7bcef473.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-dea5dd9d4edbf91b.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-ad881f1ed86c35c8.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b046be14ba8603c9.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-258d31a51f37387a.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-dbe2b7f632e2ed7d.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-825bbbfcf183d078.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-825bbbfcf183d078.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-76d7c144a7434179.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-cf1cea8893eca4e1.so` (exit code: 1)
[00:23:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:23:06] expected success, got: exit code: 101
[00:23:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:23:06] travis_fold:end:stage1-rustc

[00:23:06] travis_time:end:stage1-rustc:start=1536875516695285426,finish=1536875588448499884,duration=71753214458


[00:23:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:06] Build completed unsuccessfully in 0:18:20
[00:23:06] make: *** [all] Error 1
[00:23:06] Makefile:28: recipe for target 'all' failed
1627292 ./obj
1627256 ./obj/build
1067300 ./src
1010748 ./obj/build/x86_64-unknown-linux-gnu
---
151412 ./src/tools/clang
149124 ./src/llvm-emscripten/test
149020 ./obj/build/bootstrap/debug/incremental
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134584 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl/s-f4ryccsgkw-2eah3t-1jtu7ee7y4raz
104700 ./src/tools/lldb
99320 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
---
travis_time:end:236d84cc:start=1536875589241201981,finish=1536875589247710300,duration=6508319
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09c80c40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
