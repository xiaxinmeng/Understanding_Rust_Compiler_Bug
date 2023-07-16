plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0783
Found 499 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/library/core/src/iter/mod.rs:404: `issue` "none" mismatches the previous `issue` of "87155"
tidy error: /checkout/library/core/src/iter/adapters/map_windows.rs:8: `issue` "87155" mismatches the previous `issue` of "none"
tidy error: /checkout/library/core/src/iter/adapters/mod.rs:52: `issue` "none" mismatches the previous `issue` of "87155"
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
