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
    Finished release [optimized] target(s) in 8.87s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision [unspecified] should not specify `needs-llvm-components:` as it doesn't need `--target`
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision aarch64-apple should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision aarch64-linux should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision aarch64-windows should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision arm should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision i686 should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision riscv should specify `needs-llvm-components:` as it has `--target` set
/checkout/src/test/codegen/some-abis-do-extend-params-to-32-bits.rs: revision x86_64 should specify `needs-llvm-components:` as it has `--target` set
* highest error code: E0788
Found 505 error codes
Found 0 error(s) in error codes
Done!
