rust
    .filter(|r| r.is_err_or(|value| value.something()))
    .collect::<Result<_, _>();
