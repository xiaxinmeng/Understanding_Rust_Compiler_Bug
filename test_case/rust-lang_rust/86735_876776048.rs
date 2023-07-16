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
<Nothing changed>
+ /scripts/validate-error-codes.sh
Check if an error code explanation was removed...
Error code explanations should never be removed!
Take a look at E0001 to see how to handle it.
