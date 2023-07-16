rust
let (result, errs): (Vec<_>, Vec<_>) = toml_targets
    .into_iter()
    .map(|target| {
        target_path(
            &target,
            inferred,
            target_kind,
            package_root,
            edition,
            legacy_path,
        )
        .map(|path| (path, target))
    })
    .collect();
errors.extend(errs);
Ok(result)
