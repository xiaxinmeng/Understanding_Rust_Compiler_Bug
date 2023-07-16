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
* Read errors: 0
* Files with copyright information: 38260 / 38260
* Files with license information: 38260 / 38260

Unfortunately, your project is not compliant with version 3.0 of the REUSE Specification :-(
