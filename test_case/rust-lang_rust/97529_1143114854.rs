plain
[TIMING] compile::Sysroot { compiler: Compiler { stage: 2, host: x86_64-unknown-illumos } } -- 0.000
[TIMING] builder::Builder::sysroot_libdir::Libdir { compiler: Compiler { stage: 2, host: x86_64-unknown-illumos }, target: x86_64-unknown-illumos } -- 0.000
[TIMING] compile::Assemble { target_compiler: Compiler { stage: 2, host: x86_64-unknown-illumos } } -- 0.004
Building rustdoc for stage2 (x86_64-unknown-illumos)
error: failed to run `rustc` to learn about target-specific information
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=v0 '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Zdual-proc-macros -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Ztls-model=initial-exec --target x86_64-unknown-illumos --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: the `-Z unstable-options` flag must also be passed to enable the flag `check-cfg`

  [RUSTC-TIMING] ___ test:false 0.017
