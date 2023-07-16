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
* highest error code: E0786
Found 502 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/library/core/src/internal_macros.rs:199: `issue` "none" mismatches the corresponding lang feature `issue` of "67792"
tidy error: /checkout/library/core/src/internal_macros.rs:227: `issue` "none" mismatches the corresponding lang feature `issue` of "67792"
tidy error: /checkout/library/core/tests/iter/mod.rs: missing trailing newline
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
