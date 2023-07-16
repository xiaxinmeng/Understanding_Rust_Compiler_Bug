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
    Finished release [optimized] target(s) in 7.41s
tidy check
tidy error: invalid source: "git+https://github.com/racer-rust/racer?rev=cbca5c66b6607a340c36fcf4d5fd2166ae41e79d#cbca5c66b6607a340c36fcf4d5fd2166ae41e79d"
Checking which error codes lack tests...
* 628 error codes
* highest error code: E0786
Found 502 error codes
Found 502 error codes
Found 0 error codes with no tests
Done!
* 361 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:09
