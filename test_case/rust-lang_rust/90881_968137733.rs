plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d212d902ae8d29e31b32641096f7848a4bb35522 and 55edd2f06094d531d7de84fcd9f21cc791ebd2cf
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling num-traits v0.2.12
error: could not compile `thread_local`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name thread_local /cargo/registry/src/github.com-1ecc6299db9ec823/thread_local-1.0.1/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=ae9887d7ee5a3068 -C extra-filename=-ae9887d7ee5a3068 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern lazy_static=/checkout/obj/build/bootstrap/debug/deps/liblazy_static-1f5d3923e02f6398.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:55
make: *** [prepare] Error 1
---
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.106
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
rustc exited with signal: 11 (core dumped)
error: could not compile `memchr`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="compiler_builtins"' --cfg 'feature="core"' --cfg 'feature="rustc-dep-of-std"' -C metadata=29784148984ff36f -C extra-filename=-29784148984ff36f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/memchr-29784148984ff36f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
error: build failed
Build completed unsuccessfully in 0:00:17
cat: /tmp/toolstate/toolstates.json: No such file or directory
