rust
#![target_feature(enabled = "avx")]
fn main1() {}

fn main() { if is_x86_feature_detected!("avx") { main1() } }
