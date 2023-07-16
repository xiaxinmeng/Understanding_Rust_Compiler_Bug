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
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error: mismatched closing delimiter: `)`
   --> src/tools/compiletest/src/runtest.rs:878:75
    |
878 |             if !status.success() { panic("Program failed `{:}`", adb_path));

error: mismatched closing delimiter: `)`
   --> src/tools/compiletest/src/runtest.rs:884:75
    |
    |
884 |             if !status.success() { panic("Program failed `{:}`", adb_path));


error[E0423]: expected function, found macro `panic`
    |
    |
878 |             if !status.success() { panic("Program failed `{:}`", adb_path));
    |
    |
help: use `!` to invoke the macro
    |
878 |             if !status.success() { panic!("Program failed `{:}`", adb_path));
help: consider importing this function instead
    |
3   | use core::panicking::panic;
    |
    |

error[E0423]: expected function, found macro `panic`
    |
    |
884 |             if !status.success() { panic("Program failed `{:}`", adb_path));
    |
    |
help: use `!` to invoke the macro
    |
884 |             if !status.success() { panic!("Program failed `{:}`", adb_path));
help: consider importing this function instead
    |
3   | use core::panicking::panic;
    |
