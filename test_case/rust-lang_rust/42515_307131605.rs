rust
#![feature(cfg_target_feature, target_feature)]

#[target_feature = "+avx"]
fn should_panic() {
    #[cfg(target_feature = "avx")]
    panic!("have_avx");
}

fn main() {
    should_panic();
}
