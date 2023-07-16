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
   --> src/tools/compiletest/src/runtest.rs:878:76
    |
878 |             if !status.success() { panic!("Program failed `{:}`", adb_path));

error: mismatched closing delimiter: `)`
   --> src/tools/compiletest/src/runtest.rs:884:76
    |
    |
884 |             if !status.success() { panic!("Program failed `{:}`", adb_path));

error: aborting due to 2 previous errors

error: could not compile `compiletest`
