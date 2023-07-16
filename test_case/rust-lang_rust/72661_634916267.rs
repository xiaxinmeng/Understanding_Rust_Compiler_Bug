
   Compiling cfg-if v0.1.10
     Running `/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/bootstrap/debug/rustc --crate-name cfg_if --edition=2018 /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/vendor/cfg-if/src/lib.rs --error-format=json --js
on=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C debuginfo=0 --cfg 'feature="compiler_builtins"' --cfg 'feature="core"' --cfg 'feature="rustc-dep-of-std"' -C metadata=ea138eb22fe07b6e 
-C extra-filename=-ea138eb22fe07b6e --out-dir /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=x86_64-pc-li
nux-gnu-gcc -L dependency=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/
x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-70d65
a289aa56fa5.rmeta --extern core=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dce3d.rmeta --cap-lints allow -
Zmacro-backtrace -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Zbinary-dep-depinfo -L native=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/r
elease/build/compiler_builtins-6a53cdec3946c317/out`
rustc command: "LD_LIBRARY_PATH"="/usr/lib:/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps:/usr/lib" "/usr/bin/rustc" "--crate-name" "cfg_if" "--edition=2018" "/tmp/portage/de
v-lang/rust-1.43.1/work/rustc-1.43.1-src/vendor/cfg-if/src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=3" "-C" "debuginfo=0" "--cfg" "featu
re=\"compiler_builtins\"" "--cfg" "feature=\"core\"" "--cfg" "feature=\"rustc-dep-of-std\"" "-C" "metadata=ea138eb22fe07b6e" "-C" "extra-filename=-ea138eb22fe07b6e" "--out-dir" "/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/buil
d/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "linker=x86_64-pc-linux-gnu-gcc" "-L" "dependency=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64
-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps" "--extern" "compiler_builtins=/tmp/porta
ge/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-70d65a289aa56fa5.rmeta" "--extern" "core=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.
1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dce3d.rmeta" "--cap-lints" "allow" "-Zmacro-backtrace" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Dwarnings" "-C
prefer-dynamic" "-Zbinary-dep-depinfo" "-L" "native=/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-6a53cdec3946c317/out" "--sysroo
t" "/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-C" "debug-assertions=n" "-Z" "force-unstable-if-unmarked"
sysroot: "/tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-sysroot"
libdir: "/usr/lib"
[DEBUG rustc_metadata::creader] resolving extern crate stmt. ident: core orig_name: None
[INFO  rustc_metadata::creader] resolving crate `core`
[INFO  rustc_metadata::creader] falling back to a load
[DEBUG rustc_session::filesearch] using sysroot = /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-sysroot, triple = x86_64-unknown-linux-gnu
[INFO  rustc_metadata::locator] rmeta reading metadata from: /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dc
e3d.rmeta
[INFO  rustc_metadata::locator] reading "librustc_std_workspace_core-4f06cf76701dce3d.rmeta" => 38.059µs
[INFO  rustc_metadata::creader] register crate `rustc_std_workspace_core` (private_dep = false)
[DEBUG rustc_metadata::creader] resolving deps of external crate
[INFO  rustc_metadata::creader] resolving dep crate core hash: `f5195a85aae464c4` extra filename: `-4538f57aeeff3159`
[INFO  rustc_metadata::creader] resolving crate `core`
[INFO  rustc_metadata::creader] falling back to a load
[DEBUG rustc_session::filesearch] using sysroot = /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-sysroot, triple = x86_64-unknown-linux-gnu
[DEBUG rustc_session::filesearch] searching /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-70d65a289aa56fa5.rlib
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-70d65a289aa56fa5.rlib
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-1424704ea4a5e235.rlib
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-1424704ea4a5e235.rlib
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dce3d.rlib
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dce3d.rlib
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rlib
[INFO  rustc_metadata::locator] lib candidate: /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rlib
[DEBUG rustc_session::filesearch] picked /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rlib
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-70d65a289aa56fa5.rmeta
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-70d65a289aa56fa5.rmeta
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/compiler_builtins-70d65a289aa56fa5.d
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/compiler_builtins-70d65a289aa56fa5.d
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-1424704ea4a5e235.rmeta
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-1424704ea4a5e235.rmeta
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libc-1424704ea4a5e235.d
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libc-1424704ea4a5e235.d
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dce3d.rmeta
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4f06cf76701dce3d.rmeta
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/rustc_std_workspace_core-4f06cf76701dce3d.d
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/rustc_std_workspace_core-4f06cf76701dce3d.d
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rmeta
[INFO  rustc_metadata::locator] lib candidate: /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rmeta
[DEBUG rustc_session::filesearch] picked /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rmeta
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/core-4538f57aeeff3159.d
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/core-4538f57aeeff3159.d
[DEBUG rustc_session::filesearch] searching /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libautocfg-4ad8357f85f13c71.rlib
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libautocfg-4ad8357f85f13c71.rlib
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libcc-0cbd7187a850fe6e.rlib
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libcc-0cbd7187a850fe6e.rlib
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libautocfg-4ad8357f85f13c71.rmeta
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libautocfg-4ad8357f85f13c71.rmeta
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/autocfg-4ad8357f85f13c71.d
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/autocfg-4ad8357f85f13c71.d
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libcc-0cbd7187a850fe6e.rmeta
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/libcc-0cbd7187a850fe6e.rmeta
[DEBUG rustc_session::filesearch] testing /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/cc-0cbd7187a850fe6e.d
[DEBUG rustc_session::filesearch] rejected /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps/cc-0cbd7187a850fe6e.d
[DEBUG rustc_session::filesearch] searching /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rust-1.43.1/rustlib/x86_64-unknown-linux-gnu/lib
[INFO  rustc_metadata::locator] rmeta reading metadata from: /tmp/portage/dev-lang/rust-1.43.1/work/rustc-1.43.1-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4538f57aeeff3159.rmeta
[INFO  rustc_metadata::locator] reading "libcore-4538f57aeeff3159.rmeta" => 56.401µs
[INFO  rustc_metadata::creader] register crate `core` (private_dep = false)
[DEBUG rustc_metadata::creader] resolving deps of external crate
