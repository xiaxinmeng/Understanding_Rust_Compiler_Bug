text
; cargo +stage1 fix --broken-code
    Checking t v0.1.0 (/home/waffle/projects/repos/t)
warning: error applying suggestions to `src/main.rs`

The full error message was:

> Cannot replace slice of data that was already replaced

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

       Fixed src/main.rs (0 fixes)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
