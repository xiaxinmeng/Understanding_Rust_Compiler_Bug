rust
// does not typecheck
is_handler((|x: &str| -> &str { x }) as for<'r> fn(&'r str) -> &'r str);

// does not typecheck
is_handler(mk_handler(|x: &str| -> &str { x }));
