rust
#[cfg_attr(target_arch = "arm", target_feature(enabled = "v7")]
unsafe fn foo() {}
