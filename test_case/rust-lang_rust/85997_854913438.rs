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
     |
2522 |                 diff_output.read_to_end(&mut buf);
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: `-D unused-must-use` implied by `-D warnings`
     = note: this `Result` may be an `Err` variant, which should be handled
error: unused `std::result::Result` that must be used
    --> src/tools/compiletest/src/runtest.rs:2523:17
     |
     |
2523 |                 std::io::stderr().lock().write_all(&mut buf);
     |
     |
     = note: this `Result` may be an `Err` variant, which should be handled
error: aborting due to 2 previous errors

error: could not compile `compiletest`

