
a.rs:14:12: 14:15 error: internal compiler error: Cannot relate bound region: ReEarlyBound(13, 0, a) <= ReInfer(3)
This message reflects a bug in the Rust compiler. 
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
a.rs:14     let s = tag.get_name();
