
   Compiling rustc_plugin v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin/deprecated)
     Running `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_plugin src/librustc_plugin/deprecated/lib.rs --error-format json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=1f01ca8842d6f4ee -C extra-filename=-1f01ca8842d6f4ee --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_plugin_impl=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out`
rustc command: "LD_LIBRARY_PATH"="/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--edition=2018" "--crate-name" "rustc_plugin" "src/librustc_plugin/deprecated/lib.rs" "--error-format" "json" "--json=diagnostic-rendered-ansi" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=2" "-C" "codegen-units=1" "-C" "debuginfo=2" "-C" "metadata=1f01ca8842d6f4ee" "-C" "extra-filename=-1f01ca8842d6f4ee" "--out-dir" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "rustc_plugin_impl=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta" "-Zexternal-macro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Zunstable-options" "-Wrustc::internal" "-Cprefer-dynamic" "--cfg=parallel_compiler" "-Zbinary-dep-depinfo" "-L" "native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out" "-L" "native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out" "--sysroot" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1" "-C" "debug-assertions=n" "-Z" "force-unstable-if-unmarked"
sysroot: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
[RUSTC-TIMING] rustc_privacy test:false 294.540
     Running `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_interface src/librustc_interface/lib.rs --error-format json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=0d169ea062d01b70 -C extra-filename=-0d169ea062d01b70 --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1022b9df95eebe1a.rmeta --extern once_cell=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6557f60babc39d8f.rmeta --extern rustc=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ba8998efb57f0686.rmeta --extern rayon=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-3f14a06398fb8d27.rmeta --extern rustc_codegen_ssa=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-8ff5b3fb9091fa83.rmeta --extern rustc_codegen_utils=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-9b595da31805269f.rmeta --extern rustc_data_structures=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-985dc1d30c7b19f5.rmeta --extern rustc_errors=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1f0042c7f0e4a720.rmeta --extern rustc_incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d8d56e69686884cb.rmeta --extern rustc_lint=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-4bf5ab069f3fe1f1.rmeta --extern rustc_metadata=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-19155a172969d2ff.rmeta --extern rustc_mir=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-e9addb77fa1aa059.rmeta --extern rustc_passes=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-dd83cc2cdbdbe032.rmeta --extern rustc_plugin=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta --extern rustc_privacy=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-98cbfa24e27cade1.rmeta --extern rustc_resolve=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-6ef3ba397ea451ec.rmeta --extern rustc_traits=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-d2a356201ce29ff2.rmeta --extern rustc_typeck=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-e407089fd5307e47.rmeta --extern rustc_serialize=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c62857801cb33412.rmeta --extern smallvec=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a539e426cc999c8.rmeta --extern syntax=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f56e8cee2d0bfce1.rmeta --extern syntax_ext=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-1746e77c97422870.rmeta --extern syntax_pos=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd2603bbfec5f360.rmeta --extern tempfile=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-b1326e77fb6c7a44.rmeta -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out`
rustc command: "LD_LIBRARY_PATH"="/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--edition=2018" "--crate-name" "rustc_interface" "src/librustc_interface/lib.rs" "--error-format" "json" "--json=diagnostic-rendered-ansi" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=2" "-C" "codegen-units=1" "-C" "debuginfo=2" "-C" "metadata=0d169ea062d01b70" "-C" "extra-filename=-0d169ea062d01b70" "--out-dir" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "log=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1022b9df95eebe1a.rmeta" "--extern" "once_cell=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6557f60babc39d8f.rmeta" "--extern" "rustc=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ba8998efb57f0686.rmeta" "--extern" "rayon=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-3f14a06398fb8d27.rmeta" "--extern" "rustc_codegen_ssa=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-8ff5b3fb9091fa83.rmeta" "--extern" "rustc_codegen_utils=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-9b595da31805269f.rmeta" "--extern" "rustc_data_structures=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-985dc1d30c7b19f5.rmeta" "--extern" "rustc_errors=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1f0042c7f0e4a720.rmeta" "--extern" "rustc_incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d8d56e69686884cb.rmeta" "--extern" "rustc_lint=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-4bf5ab069f3fe1f1.rmeta" "--extern" "rustc_metadata=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-19155a172969d2ff.rmeta" "--extern" "rustc_mir=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-e9addb77fa1aa059.rmeta" "--extern" "rustc_passes=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-dd83cc2cdbdbe032.rmeta" "--extern" "rustc_plugin=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta" "--extern" "rustc_privacy=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-98cbfa24e27cade1.rmeta" "--extern" "rustc_resolve=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-6ef3ba397ea451ec.rmeta" "--extern" "rustc_traits=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-d2a356201ce29ff2.rmeta" "--extern" "rustc_typeck=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-e407089fd5307e47.rmeta" "--extern" "rustc_serialize=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c62857801cb33412.rmeta" "--extern" "smallvec=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a539e426cc999c8.rmeta" "--extern" "syntax=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f56e8cee2d0bfce1.rmeta" "--extern" "syntax_ext=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-1746e77c97422870.rmeta" "--extern" "syntax_pos=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd2603bbfec5f360.rmeta" "--extern" "tempfile=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-b1326e77fb6c7a44.rmeta" "-Zexternal-macro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Zunstable-options" "-Wrustc::internal" "-Cprefer-dynamic" "--cfg=parallel_compiler" "-Zbinary-dep-depinfo" "-L" "native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out" "-L" "native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out" "--sysroot" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1" "-C" "debug-assertions=n" "-Z" "force-unstable-if-unmarked"
sysroot: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
[RUSTC-TIMING] rustc_plugin_impl test:false 434.101
error: internal compiler error: src/librustc/ich/impls_ty.rs:100: StableHasher: unexpected region '_#4r

thread '<unnamed>' panicked at 'Box<Any>', src/librustc_errors/lib.rs:912:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1187
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:407
  13: rustc_errors::HandlerInner::bug
             at src/librustc_errors/lib.rs:912
  14: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:684
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:36
  16: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:1982
  17: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1932
  18: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:1982
  19: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  20: rustc::util::bug::bug_fmt
             at src/librustc/util/bug.rs:12
  21: rustc::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::sty::RegionKind>::hash_stable
             at src/librustc/ich/impls_ty.rs:100
  22: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  23: rustc::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::sty::TyKind>::hash_stable
             at src/librustc/ty/sty.rs:89
  24: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  25: rustc::ty::context::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_ich_StableHashingContext_ctx_FOR_GeneratorInteriorTypeCause::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::context::GeneratorInteriorTypeCause>::hash_stable
             at src/librustc/ty/context.rs:309
  26: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:289
  27: <alloc::vec::Vec<T> as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:297
  28: <rustc::ty::context::TypeckTables as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable::{{closure}}
             at src/librustc/ty/context.rs:810
  29: rustc::ich::hcx::StableHashingContext::with_node_id_hashing_mode
             at src/librustc/ich/hcx.rs:129
  30: <rustc::ty::context::TypeckTables as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
             at src/librustc/ty/context.rs:769
  31: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  32: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  33: rustc::dep_graph::graph::hash_result
             at src/librustc/dep_graph/graph.rs:88
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::hash_result
             at src/librustc/ty/query/plumbing.rs:1011
  35: core::ops::function::FnOnce::call_once
             at ./src/libcore/ops/function.rs:227
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:286
  37: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
  40: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  41: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  42: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  43: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
  45: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
  46: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
  47: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
  48: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
  49: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
  52: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
  55: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
  56: rustc::ty::query::__query_compute::typeck_tables_of
             at ./src/librustc/ty/query/plumbing.rs:954
  57: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
             at ./src/librustc/ty/query/plumbing.rs:995
  58: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
  59: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  60: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  61: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  62: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  63: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
  64: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
  65: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
  66: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
  67: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
  68: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
  70: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
  71: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  72: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  73: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  74: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  75: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
  76: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
  77: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
  78: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
  79: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
  80: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
  81: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
  82: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
  83: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
  84: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
  85: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
  86: rustc::ty::query::TyCtxtAt::typeck_tables_of
             at ./src/librustc/ty/query/plumbing.rs:1080
  87: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::typeck_tables_of
             at ./src/librustc/ty/query/plumbing.rs:1072
  88: rustc_typeck::collect::checked_type_of
             at src/librustc_typeck/collect.rs:1362
  89: rustc_typeck::collect::type_of
             at src/librustc_typeck/collect.rs:1144
  90: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
  91: rustc::ty::query::__query_compute::type_of
             at ./src/librustc/ty/query/plumbing.rs:954
  92: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
             at ./src/librustc/ty/query/plumbing.rs:995
  93: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
  94: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  95: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  96: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  97: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  98: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
  99: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 100: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 101: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 102: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
 103: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
 104: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
 105: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
 106: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 107: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 108: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 109: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 110: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
 111: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
 112: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 113: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 114: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 115: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
 116: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
 117: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
 118: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
 119: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
 120: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
 121: rustc::ty::query::TyCtxtAt::type_of
             at ./src/librustc/ty/query/plumbing.rs:1080
 122: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::type_of
             at ./src/librustc/ty/query/plumbing.rs:1072
 123: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_expr
             at src/librustc_typeck/collect.rs:141
 124: rustc::hir::intravisit::walk_expr
             at ./<::syntax::visit::walk_list macros>:2
 125: rustc::hir::intravisit::walk_local
             at ./<::syntax::visit::walk_list macros>:2
 126: rustc::hir::intravisit::walk_block
             at ./<::syntax::visit::walk_list macros>:2
 127: rustc::hir::intravisit::Visitor::visit_fn
             at ./src/librustc/hir/intravisit.rs:293
 128: rustc::hir::intravisit::walk_item
             at ./src/librustc/hir/intravisit.rs:485
 129: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
             at src/librustc_typeck/collect.rs:114
 130: rustc::hir::map::Map::visit_item_likes_in_module
             at ./src/librustc/hir/map/mod.rs:578
 131: rustc_typeck::collect::collect_mod_item_types
             at src/librustc_typeck/collect.rs:57
 132: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
 133: rustc::ty::query::__query_compute::collect_mod_item_types
             at ./src/librustc/ty/query/plumbing.rs:954
 134: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
             at ./src/librustc/ty/query/plumbing.rs:995
 135: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
 136: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 137: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 138: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 139: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 140: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
 141: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 142: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 143: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 144: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
 145: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
 146: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
 147: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
 148: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 149: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 150: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 151: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 152: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
 153: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
 154: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 155: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 156: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 157: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
 158: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
 159: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
 160: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
 161: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
 162: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
 163: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at ./src/librustc/ty/query/plumbing.rs:619
 164: rustc::ty::query::TyCtxtEnsure::collect_mod_item_types
             at ./src/librustc/ty/query/plumbing.rs:1031
 165: rustc_typeck::check_crate::{{closure}}::{{closure}}
             at src/librustc_typeck/lib.rs:306
 166: rustc::util::common::time_ext
             at ./src/librustc/util/common.rs:116
 167: rustc::util::common::time
             at ./src/librustc/util/common.rs:110
 168: rustc_typeck::check_crate::{{closure}}
             at src/librustc_typeck/lib.rs:304
 169: rustc::session::Session::track_errors
             at ./src/librustc/session/mod.rs:334
 170: rustc_typeck::check_crate
             at src/librustc_typeck/lib.rs:303
 171: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:915
 172: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
 173: rustc::ty::query::__query_compute::analysis
             at ./src/librustc/ty/query/plumbing.rs:954
 174: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
 175: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 176: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 177: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 178: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 179: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
 180: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 181: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 182: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 183: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
 184: rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at ./src/librustc/dep_graph/graph.rs:387
 185: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:559
 186: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
 187: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 188: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 189: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 190: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 191: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
 192: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
 193: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 194: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 195: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 196: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
 197: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
 198: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
 199: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
 200: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
 201: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
 202: rustc::ty::query::TyCtxtAt::analysis
             at ./src/librustc/ty/query/plumbing.rs:1080
 203: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis
             at ./src/librustc/ty/query/plumbing.rs:1072
 204: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:377
 205: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}::{{closure}}
             at ./src/librustc_interface/passes.rs:809
 206: rustc::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc/ty/context.rs:1886
 207: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 208: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 209: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 210: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 211: rustc::ty::context::tls::enter_global
             at ./src/librustc/ty/context.rs:1885
 212: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}
             at ./src/librustc_interface/passes.rs:809
 213: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:21
 214: rustc_interface::passes::create_global_ctxt::{{closure}}
             at src/librustc_interface/passes.rs:873
 215: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at ./src/librustc_data_structures/box_region.rs:52
 216: rustc_interface::passes::BoxedGlobalCtxt::access
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:24
 217: rustc_interface::passes::BoxedGlobalCtxt::enter
             at ./src/librustc_interface/passes.rs:809
 218: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:377
 219: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:122
 220: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:141
 221: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:219
 222: rustc_rayon_core::thread_pool::ThreadPool::install::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:160
 223: rustc_rayon_core::registry::Registry::in_worker_cold::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:395
 224: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 225: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:315
 226: std::panicking::try::do_call
             at ./src/libstd/panicking.rs:292
 227: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 228: std::panicking::try
             at ./src/libstd/panicking.rs:271
 229: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 230: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 231: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 232: rustc_rayon_core::job::JobRef::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:62
 233: rustc_rayon_core::registry::WorkerThread::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:657
 234: rustc_rayon_core::registry::WorkerThread::wait_until_cold
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:637
 235: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:235
 236: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 237: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:235
 238: rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}
             at ./src/librustc/ty/context.rs:1842
 239: std::thread::local::LocalKey<T>::try_with
             at ./src/libstd/thread/local.rs:262
 240: std::thread::local::LocalKey<T>::with
             at ./src/libstd/thread/local.rs:239
 241: rustc::ty::context::tls::with_thread_locals::{{closure}}
             at ./src/librustc/ty/context.rs:1834
 242: std::thread::local::LocalKey<T>::try_with
             at ./src/libstd/thread/local.rs:262
 243: std::thread::local::LocalKey<T>::with
             at ./src/libstd/thread/local.rs:239
 244: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:234
 245: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 246: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:230
 247: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 248: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:229
 249: rustc_rayon_core::thread_pool::ThreadPool::scoped_pool::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:104
 250: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 251: std::panicking::try
             at ./src/libstd/panicking.rs:271
 252: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 253: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 254: rustc_rayon_core::registry::main_loop
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:747
 255: rustc_rayon_core::registry::Registry::new::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:145
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-dev (702b45e40 2019-10-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C incremental -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `passes::configure_and_expand`
#1 [typeck_tables_of] processing `passes::configure_and_expand::{{closure}}#0`
#2 [type_of] processing `passes::configure_and_expand::{{closure}}#0`
#3 [collect_mod_item_types] collecting item types in module `passes`
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

[RUSTC-TIMING] rustc_plugin test:false 121.763
[RUSTC-TIMING] rustc_typeck test:false 1551.423
[RUSTC-TIMING] rustc_interface test:false 25.942
error: could not compile `rustc_interface`.

Caused by:
  process didn't exit successfully: `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_interface src/librustc_interface/lib.rs --error-format json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=0d169ea062d01b70 -C extra-filename=-0d169ea062d01b70 --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1022b9df95eebe1a.rmeta --extern once_cell=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6557f60babc39d8f.rmeta --extern rustc=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ba8998efb57f0686.rmeta --extern rayon=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-3f14a06398fb8d27.rmeta --extern rustc_codegen_ssa=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-8ff5b3fb9091fa83.rmeta --extern rustc_codegen_utils=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-9b595da31805269f.rmeta --extern rustc_data_structures=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-985dc1d30c7b19f5.rmeta --extern rustc_errors=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1f0042c7f0e4a720.rmeta --extern rustc_incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d8d56e69686884cb.rmeta --extern rustc_lint=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-4bf5ab069f3fe1f1.rmeta --extern rustc_metadata=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-19155a172969d2ff.rmeta --extern rustc_mir=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-e9addb77fa1aa059.rmeta --extern rustc_passes=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-dd83cc2cdbdbe032.rmeta --extern rustc_plugin=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta --extern rustc_privacy=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-98cbfa24e27cade1.rmeta --extern rustc_resolve=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-6ef3ba397ea451ec.rmeta --extern rustc_traits=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-d2a356201ce29ff2.rmeta --extern rustc_typeck=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-e407089fd5307e47.rmeta --extern rustc_serialize=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c62857801cb33412.rmeta --extern smallvec=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a539e426cc999c8.rmeta --extern syntax=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f56e8cee2d0bfce1.rmeta --extern syntax_ext=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-1746e77c97422870.rmeta --extern syntax_pos=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd2603bbfec5f360.rmeta --extern tempfile=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-b1326e77fb6c7a44.rmeta -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_passes test:false 502.819
[RUSTC-TIMING] rustc_save_analysis test:false 122.828
