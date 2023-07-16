
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist --stage 2
Caused by:
Build completed unsuccessfully in 0:00:01
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --cfg=bootstrap -Zmacro-backtrace -Zosx-rpath-install-name '-Clink-args=-Wl,-rpath,@loader_path/../lib' -Zrun-dsymutil=no -Ztls-model=initial-exec --target x86_64-apple-darwin --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit code: 1)
== clock drift check ==
  --- stderr
  local time: Wed Feb 17 02:16:27 UTC 2021
  error: unknown debugging option: `run-dsymutil`
