rust
    let args = env::args_os()
        .skip(1)
        .filter(|arg| match arg.to_str().and_then(|s| s.strip_prefix("-rustc-lld-flavor=")) {
            Some(suffix) => {
                dbg!(arg);
                flavor = Some(suffix.to_string());
                return false
            }
            None => {dbg!(arg); return true},
        })
        .collect::<Vec<_>>();
