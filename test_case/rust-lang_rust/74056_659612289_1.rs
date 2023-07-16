rust
    panic_fmt(fmt::Arguments::new_v1(&[""], &[fmt::ArgumentV1::new(&expr, |&e, f| f.write_str(e))]));
