rust
is_handler(static_handler as for<'r> fn(&'r str) -> &'r str);

is_handler((|_| "hi") as for<'r> fn(&'r str) -> &'r str);

is_handler((|x| x) as for<'r> fn(&'r str) -> &'r str);
