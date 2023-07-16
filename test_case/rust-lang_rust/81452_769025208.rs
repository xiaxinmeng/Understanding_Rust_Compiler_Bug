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
   Compiling tracing v0.1.19
   Compiling tracing-subscriber v0.2.13
   Compiling rustfix v0.5.1
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error: unused `ExitStatus` that must be used
    |
    |
872 | /             Command::new(adb_path)
873 | |                 .arg("push")
874 | |                 .arg(&exe_file)
875 | |                 .arg(&self.config.adb_test_dir)
876 | |                 .status()
877 | |                 .unwrap_or_else(|_| panic!("failed to exec `{:?}`", adb_path));
    |
    |
    = note: `-D unused-must-use` implied by `-D warnings`
    = note: this ExitStatus might represent an error that should be checked and handled

error: unused `ExitStatus` that must be used
    |
    |
879 | /             Command::new(adb_path)
880 | |                 .args(&["forward", "tcp:5039", "tcp:5039"])
881 | |                 .status()
882 | |                 .unwrap_or_else(|_| panic!("failed to exec `{:?}`", adb_path));
    |
    = note: this ExitStatus might represent an error that should be checked and handled

error: aborting due to 2 previous errors
