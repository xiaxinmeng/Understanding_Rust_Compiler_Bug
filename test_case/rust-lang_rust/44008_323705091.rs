Rust
[00:28:14]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)

[00:28:14] error: stability attributes may not be used outside of the standard library

[00:28:14]    --> /checkout/src/librustc_plugin/registry.rs:129:5

[00:28:14]     |

[00:28:14] 129 |     #[unstable(feature = "rustc_private", issue = "27812")]

[00:28:14]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

[00:28:14] 

[00:28:14] error: stability attributes may not be used outside of the standard library

[00:28:14]    --> /checkout/src/librustc_plugin/registry.rs:130:5

[00:28:14]     |

[00:28:14] 130 |     #[rustc_deprecated(since = "1.15.0", reason = "replaced by macros 1.1 (RFC 1861)")]

[00:28:14]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

[00:28:14] 

[00:28:15] error: aborting due to 2 previous errors
