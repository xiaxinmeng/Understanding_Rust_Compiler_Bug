plain

#                                                                          2.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating git repository `https://github.com/fortanix/rust-sgx.git`
---
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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 12.27s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: invalid source: "git+https://github.com/fortanix/rust-sgx.git?branch=raoul/tcs_control#495f16cb6cd0d8f6a7b25ade5196a96a0d22fe35"
* 625 error codes
* highest error code: E0783
Found 499 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
* 340 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:15
