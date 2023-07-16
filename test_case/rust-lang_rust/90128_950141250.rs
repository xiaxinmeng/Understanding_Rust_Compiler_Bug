plain
   Compiling merge v0.1.0
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 41.83s
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: failed to run `rustc` to learn about target-specific information
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --cfg=bootstrap -Csymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: unknown codegen option: `symbol-mangling-version`
Build completed unsuccessfully in 0:00:57
