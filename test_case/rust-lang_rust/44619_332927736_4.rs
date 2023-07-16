rust
// does not typecheck
is_handler((|x: &str| x) as for<'r> fn(&'r str) -> &'r str);

// typechecks
is_handler(mk_handler(|x: &str| x));
