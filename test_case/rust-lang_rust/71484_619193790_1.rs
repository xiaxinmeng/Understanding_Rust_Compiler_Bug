rust
    let mut paths = vec![&build_targets.include];
    paths.extend(&build_targets.static_lib);
    paths.extend(&build_targets.shared_lib);
