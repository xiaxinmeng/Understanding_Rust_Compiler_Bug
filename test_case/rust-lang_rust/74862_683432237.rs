plain
DirectMap1G:    53477376 kB
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
[TIMING] PlainSourceTarball -- 1.798
Distcheck
error: failed to read `/checkout/obj/build/tmp/dist/rustc-nightly-src/compiler/rustc/Cargo.toml`
Caused by:
Caused by:
  No such file or directory (os error 2)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor" "--sync" "/checkout/./src/tools/rust-analyzer/Cargo.toml"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test distcheck
Build completed unsuccessfully in 0:00:08
Build completed unsuccessfully in 0:00:08
== clock drift check ==
  local time: Sun Aug 30 15:12:07 UTC 2020
  network time: Sun, 30 Aug 2020 15:12:07 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4478) (node)
Terminate orphan process: pid (4487) (python)
