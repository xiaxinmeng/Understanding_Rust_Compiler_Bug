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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
/checkout/src/test/ui/asm/issue-85247.rs: revision [unspecified] should not specify `needs-llvm-components:` as it doesn't need `--target`
/checkout/src/test/ui/asm/issue-85247.rs: revision ropi should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/ui/asm/issue-85247.rs: revision rwpi should specify `needs-llvm-components:` as it has `--target` set
Found 0 error(s) in error codes
Done!
* 366 features
some tidy checks failed
