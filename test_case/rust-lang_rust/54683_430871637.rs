
[01:53:47] ---- fix::broken_fixes_backed_out stdout ----
[01:53:47] running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
[01:53:47] running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe fix --allow-no-vcs`
[01:53:47] thread 'fix::broken_fixes_backed_out' panicked at '
[01:53:47] Expected: execs
[01:53:47]     but: expected to find:
[01:53:47] warning: failed to automatically apply fixes suggested by rustc to crate `bar`
[01:53:47] 
[01:53:47] after fixes were automatically applied the compiler reported errors within these files:
[01:53:47] 
[01:53:47]   * src/lib.rs
[01:53:47] 
[01:53:47] This likely indicates a bug in either rustc or cargo itself,
[01:53:47] and we would appreciate a bug report! You're likely to see 
[01:53:47] a number of compiler warnings after this message which cargo
[01:53:47] attempted to fix but failed. If you could open an issue at
[01:53:47] https://github.com/rust-lang/cargo/issues
[01:53:47] quoting the full output of this command we'd be very appreciative!
[01:53:47] 
[01:53:47] did not find in output:
[01:53:47]    Compiling bar v0.1.0 (C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t563\foo\bar)
[01:53:47] error: expected one of `!` or `::`, found `rust`
[01:53:47]  --> src\lib.rs:1:5
[01:53:47]   |
[01:53:47] 1 | not rust code
[01:53:47]   |     ^^^^ expected one of `!` or `::` here
[01:53:47] 
[01:53:47] error: aborting due to previous error
[01:53:47] 
[01:53:47] error: Could not compile `bar`.
[01:53:47] warning: build failed, waiting for other jobs to finish...
[01:53:47]       Fixing src\lib.rs (1 fix)
[01:53:47] error: build failed
[01:53:47] ', tools\cargo\tests\testsuite\support\mod.rs:741:13
[01:53:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
