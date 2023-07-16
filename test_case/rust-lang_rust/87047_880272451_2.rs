rust
let mut result = Vec::new();
for target in toml_targets {
    let path = target_path(
        &target,
        inferred,
        target_kind,
        package_root,
        edition,
        legacy_path,
    );
    let path = match path {
        Ok(path) => path,
        Err(e) => {
            errors.push(e);
            continue;
        }
    };
    result.push((path, target));
}
Ok(result)
