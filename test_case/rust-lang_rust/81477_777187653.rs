plain
#                                                                          2.4%
##############################################                            64.4%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-01-28/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating git repository `https://github.com/webdesus/fs_extra`
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
Found 436 error codes
Found 0 error codes with no tests
Done!
* 319 features
tidy error: invalid source: "git+https://github.com/webdesus/fs_extra#752fc4d1e6e4eff3e506bffe7196e2a537c6c52b"
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

