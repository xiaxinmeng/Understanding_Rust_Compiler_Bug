plain
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
error: use `Session::split_debuginfo` instead of this field
   --> compiler/rustc_interface/src/tests.rs:564:24
    |
561 | /     macro_rules! tracked {
562 | |         ($name: ident, $non_default_value: expr) => {
563 | |             opts = reference.clone();
564 | |             assert_ne!(opts.cg.$name, $non_default_value);
...   |
567 | |         };
568 | |     }
    | |_____- in this expansion of `tracked!`
    | |_____- in this expansion of `tracked!`
...
599 |       tracked!(split_debuginfo, Some(SplitDebuginfo::Packed));
    |
    |
    = note: `#[deny(rustc::bad_opt_access)]` on by default
error: use `Session::split_debuginfo` instead of this field
   --> compiler/rustc_interface/src/tests.rs:565:13
    |
561 | /     macro_rules! tracked {
561 | /     macro_rules! tracked {
562 | |         ($name: ident, $non_default_value: expr) => {
563 | |             opts = reference.clone();
564 | |             assert_ne!(opts.cg.$name, $non_default_value);
565 | |             opts.cg.$name = $non_default_value;
    | |             ^^^^^^^^^^^^^
566 | |             assert_different_hash(&reference, &opts);
568 | |     }
    | |_____- in this expansion of `tracked!`
...
...
599 |       tracked!(split_debuginfo, Some(SplitDebuginfo::Packed));

error: could not compile `rustc_interface` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
 finished in 35.997 seconds
