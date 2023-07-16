plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.68s
tidy check
Checking rustdoc IDs...
Checking which error codes lack tests...
Checking which error codes lack tests...
=> ID `deref-methods` is unused, it should be removed from `init_id_map` in file `librustdoc/html/markdown.rs`
* highest error code: E0786
Found 502 error codes
Found 0 error codes with no tests
Done!
Done!
* 358 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
