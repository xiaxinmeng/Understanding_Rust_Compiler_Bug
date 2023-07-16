plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
thread 'main' panicked at '
Failed to run:
"build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "cc" "--edition=2018" "/cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.69/src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "debug-assertions=off" "-C" "metadata=043ec11256d7f709" "-C" "extra-filename=-043ec11256d7f709" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--cap-lints" "allow" "-Z" "binary-dep-depinfo" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Wsemicolon_in_expressions_from_macros" "-Dwarnings" "--cfg=bootstrap" "-Zunstable-options" "--check-cfg=values(bootstrap)"
-------------: Os { code: 2, kind: NotFound, message: "No such file or directory" }', bin/rustc.rs:189:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Failed to run:
Failed to run:
"build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "core" "--edition=2021" "library/core/src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "opt-level=3" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "metadata=19a6c7cb160f2c97" "-C" "extra-filename=-19a6c7cb160f2c97" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--cfg=bootstrap" "-Csymbol-mangling-version=legacy" "-Zunstable-options" "-Zmacro-backtrace" "-Clink-args=-Wl,-z,origin" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Zunstable-options" "-Csplit-debuginfo=off" "-Cprefer-dynamic" "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")" "-Z" "binary-dep-depinfo" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Wsemicolon_in_expressions_from_macros" "-Dwarnings" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-Z" "force-unstable-if-unmarked"
-------------: Os { code: 2, kind: NotFound, message: "No such file or directory" }', bin/rustc.rs:189:37
error: could not compile `cc`
warning: build failed, waiting for other jobs to finish...
thread 'main' panicked at '
Failed to run:
Failed to run:
"build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "build_script_build" "--edition=2018" "/cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/build.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "bin" "--emit=dep-info,link" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "debug-assertions=off" "--cfg" "feature=\"compiler_builtins\"" "--cfg" "feature=\"core\"" "--cfg" "feature=\"rustc-dep-of-std\"" "-C" "metadata=931bb27d0a703ea3" "-C" "extra-filename=-931bb27d0a703ea3" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/memchr-931bb27d0a703ea3" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--cap-lints" "allow" "-Z" "binary-dep-depinfo" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Wsemicolon_in_expressions_from_macros" "-Dwarnings" "--cfg=bootstrap" "-Zunstable-options" "--check-cfg=values(bootstrap)"
-------------: Os { code: 2, kind: NotFound, message: "No such file or directory" }', bin/rustc.rs:189:37
error: could not compile `core`
error: could not compile `memchr`
thread 'main' panicked at '
Failed to run:
Failed to run:
"build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "build_script_build" "/cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.126/build.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "bin" "--emit=dep-info,link" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "debug-assertions=off" "--cfg" "feature=\"align\"" "--cfg" "feature=\"rustc-dep-of-std\"" "--cfg" "feature=\"rustc-std-workspace-core\"" "-C" "metadata=a89f6afc96c12b80" "-C" "extra-filename=-a89f6afc96c12b80" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/libc-a89f6afc96c12b80" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--cap-lints" "allow" "-Z" "binary-dep-depinfo" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Wsemicolon_in_expressions_from_macros" "-Dwarnings" "--cfg=bootstrap" "-Zunstable-options" "--check-cfg=values(bootstrap)"
-------------: Os { code: 2, kind: NotFound, message: "No such file or directory" }', bin/rustc.rs:189:37
error: could not compile `libc`
thread 'main' panicked at '
Failed to run:
Failed to run:
"build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "build_script_build" "--edition=2021" "library/std/build.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "bin" "--emit=dep-info,link" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "debug-assertions=off" "--cfg" "feature=\"addr2line\"" "--cfg" "feature=\"backtrace\"" "--cfg" "feature=\"compiler-builtins-c\"" "--cfg" "feature=\"gimli-symbolize\"" "--cfg" "feature=\"miniz_oxide\"" "--cfg" "feature=\"object\"" "--cfg" "feature=\"panic_unwind\"" "--cfg" "feature=\"std_detect_dlsym_getauxval\"" "--cfg" "feature=\"std_detect_file_io\"" "-C" "metadata=903737d40773d7d6" "-C" "extra-filename=-903737d40773d7d6" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/std-903737d40773d7d6" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "-Z" "binary-dep-depinfo" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Wsemicolon_in_expressions_from_macros" "-Dwarnings" "--cfg=bootstrap" "-Zunstable-options" "--check-cfg=values(bootstrap)"
-------------: Os { code: 2, kind: NotFound, message: "No such file or directory" }', bin/rustc.rs:189:37
error: could not compile `std`
Build completed unsuccessfully in 0:01:30
