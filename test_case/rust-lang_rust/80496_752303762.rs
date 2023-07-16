plain
configure: rust.ignore-git      := False
configure: build.print-step-timings := True
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: dist.compression-formats := ['xz']
configure: 
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
---
Dist rust-src-nightly
 finished in 5.312 seconds
[TIMING] Src -- 5.424

gzip: stdin: not in gzip format
tar: Child returned status 1
tar: Error is not recoverable: exiting now


command did not execute successfully: "tar" "-xzf" "/checkout/obj/build/dist/rustc-nightly-src.tar.xz" "--strip-components=1"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test distcheck
Build completed unsuccessfully in 0:01:46
