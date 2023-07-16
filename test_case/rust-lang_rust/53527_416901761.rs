rust
fn is_nightly() -> bool {
    // Whether this is a feature-staged build, i.e. on the beta or stable channel
    let disable_unstable_features = option_env!("CFG_DISABLE_UNSTABLE_FEATURES").is_some();
    // Whether we should enable unstable features for bootstrapping
    let bootstrap = env::var("RUSTC_BOOTSTRAP").is_ok();

    bootstrap || !disable_unstable_features
}
