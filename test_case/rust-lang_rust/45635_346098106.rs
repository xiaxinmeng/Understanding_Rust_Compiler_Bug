diff
 error: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
   --> $DIR/issue-44953.rs:16:14
    |
 16 | #[macro_use] extern crate log;
    |              ^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
 
 error: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
   --> $DIR/issue-44953.rs:19:5
    |
 19 |     info!("This is a log message.");
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
-   = note: this error originates in a macro outside of the current crate
+   = note: this error originates in a macro outside of the current crate (run with -Z external-macro-backtrace for more info)
 
 error: aborting due to 2 previous errors
