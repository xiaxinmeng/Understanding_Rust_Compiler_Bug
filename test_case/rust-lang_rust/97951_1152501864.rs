rust
fn ice_on_inference_failure(_: impl Into<i32>) {}

fn test() {
    let s: &str = clap::Command::new("").get_matches().value_of("").unwrap();
    let p = s.parse().unwrap();
    ice_on_inference_failure(p);
}
