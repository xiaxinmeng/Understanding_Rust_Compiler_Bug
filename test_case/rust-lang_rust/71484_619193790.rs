rust
    let mut paths = vec![&build_targets.include];
    build_targets.static_lib.as_ref().map(|p| paths.push(p));
    build_targets.shared_lib.as_ref().map(|p| paths.push(p));
