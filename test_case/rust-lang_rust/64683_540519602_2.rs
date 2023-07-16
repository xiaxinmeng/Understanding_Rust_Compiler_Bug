rust
fn foo() {
    if cfg_feature_enabled!("avx") {
      unsafe { foo_avx() } // not const-checked
    } else if cfg_feature_enabled!("sse4") {
      unsafe { foo_sse4() } // not const-checked
    } else {
      foo_impl() // use the default version
    }
}
