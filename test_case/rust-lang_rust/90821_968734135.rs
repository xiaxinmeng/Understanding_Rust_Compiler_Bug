plain
   Compiling cmake v0.1.44
error: could not compile `cc`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name cc --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.69/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=9022b5d7a536baba -C extra-filename=-9022b5d7a536baba --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:55
make: *** [prepare] Error 1
---
   Compiling globset v0.4.5
error: could not compile `regex`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name regex --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.5.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="aho-corasick"' --cfg 'feature="default"' --cfg 'feature="memchr"' --cfg 'feature="perf"' --cfg 'feature="perf-cache"' --cfg 'feature="perf-dfa"' --cfg 'feature="perf-inline"' --cfg 'feature="perf-literal"' --cfg 'feature="std"' --cfg 'feature="unicode"' --cfg 'feature="unicode-age"' --cfg 'feature="unicode-bool"' --cfg 'feature="unicode-case"' --cfg 'feature="unicode-gencat"' --cfg 'feature="unicode-perl"' --cfg 'feature="unicode-script"' --cfg 'feature="unicode-segment"' -C metadata=b0c184cb911fb168 -C extra-filename=-b0c184cb911fb168 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern aho_corasick=/checkout/obj/build/bootstrap/debug/deps/libaho_corasick-6acdc16a2f6d6338.rmeta --extern memchr=/checkout/obj/build/bootstrap/debug/deps/libmemchr-9d65e5e3abdde0ba.rmeta --extern regex_syntax=/checkout/obj/build/bootstrap/debug/deps/libregex_syntax-74207c8016af6cf7.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:03
make: *** [prepare] Error 1
---
[RUSTC-TIMING] adler test:false 0.155
[RUSTC-TIMING] unwind test:false 0.075
[RUSTC-TIMING] libc test:false 0.721
[RUSTC-TIMING] compiler_builtins test:false 0.897
rustc exited with signal: 11 (core dumped)
error: could not compile `compiler_builtins`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.49/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=1 --cfg 'feature="c"' --cfg 'feature="cc"' --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=6bdc4f8396c65e15 -C extra-filename=-6bdc4f8396c65e15 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-fd44d8b17c2359b1.rmeta --cap-lints allow --cfg=bootstrap -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-399cb2af88d0d65a/out --cfg 'feature="unstable"' --cfg '__absvdi2="optimized-c"' --cfg '__absvsi2="optimized-c"' --cfg '__absvti2="optimized-c"' --cfg '__addvdi3="optimized-c"' --cfg '__addvsi3="optimized-c"' --cfg '__addvti3="optimized-c"' --cfg '__clzdi2="optimized-c"' --cfg '__clzsi2="optimized-c"' --cfg '__clzti2="optimized-c"' --cfg '__cmpdi2="optimized-c"' --cfg '__cmpti2="optimized-c"' --cfg '__ctzdi2="optimized-c"' --cfg '__ctzsi2="optimized-c"' --cfg '__ctzti2="optimized-c"' --cfg '__divdc3="optimized-c"' --cfg '__divsc3="optimized-c"' --cfg '__divxc3="optimized-c"' --cfg '__extendhfsf2="optimized-c"' --cfg '__ffsti2="optimized-c"' --cfg '__floatdisf="optimized-c"' --cfg '__floatdixf="optimized-c"' --cfg '__floatundidf="optimized-c"' --cfg '__floatundisf="optimized-c"' --cfg '__floatundixf="optimized-c"' --cfg '__int_util="optimized-c"' --cfg '__muldc3="optimized-c"' --cfg '__mulsc3="optimized-c"' --cfg '__mulvdi3="optimized-c"' --cfg '__mulvsi3="optimized-c"' --cfg '__mulvti3="optimized-c"' --cfg '__mulxc3="optimized-c"' --cfg '__negdf2="optimized-c"' --cfg '__negdi2="optimized-c"' --cfg '__negsf2="optimized-c"' --cfg '__negti2="optimized-c"' --cfg '__negvdi2="optimized-c"' --cfg '__negvsi2="optimized-c"' --cfg '__negvti2="optimized-c"' --cfg '__paritydi2="optimized-c"' --cfg '__paritysi2="optimized-c"' --cfg '__parityti2="optimized-c"' --cfg '__popcountdi2="optimized-c"' --cfg '__popcountsi2="optimized-c"' --cfg '__popcountti2="optimized-c"' --cfg '__powixf2="optimized-c"' --cfg '__subvdi3="optimized-c"' --cfg '__subvsi3="optimized-c"' --cfg '__subvti3="optimized-c"' --cfg '__truncdfhf2="optimized-c"' --cfg '__truncdfsf2="optimized-c"' --cfg '__truncsfhf2="optimized-c"' --cfg '__ucmpdi2="optimized-c"' --cfg '__ucmpti2="optimized-c"' --cfg 'apple_versioning="optimized-c"' -l static=compiler-rt` (exit status: 254)
[RUSTC-TIMING] memchr test:false 1.375
[RUSTC-TIMING] rustc_demangle test:false 1.515
[RUSTC-TIMING] alloc test:false 2.293
[RUSTC-TIMING] core test:false 16.420
