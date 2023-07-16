plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
<Nothing changed>
+ /scripts/validate-error-codes.sh
Check if an error code explanation was removed...
Error code explanations should never be removed!
Take a look at E0001 to see how to handle it.
