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
tidy error: could not find exception package `fuchsia-zircon-sys`
Remove from EXCEPTIONS list if it is no longer used.
tidy error: could not find exception package `fuchsia-zircon`
Remove from EXCEPTIONS list if it is no longer used.
tidy error: could not find allowed package `fuchsia-zircon`
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
tidy error: could not find allowed package `fuchsia-zircon-sys`
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
tidy error: could not find allowed package `kernel32-sys`
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
tidy error: could not find allowed package `winapi-build`
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
