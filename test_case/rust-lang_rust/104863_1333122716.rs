
error[E0277]: `impl EarlyLintPass + 'static` cannot be sent between threads safely
    |
378 |     passes.push(Box::new(builtin_lints));
378 |     passes.push(Box::new(builtin_lints));
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ `impl EarlyLintPass + 'static` cannot be sent between threads safely
    |
    = note: required for the cast from `impl EarlyLintPass + 'static` to the object type `dyn EarlyLintPass + Send`
    |
    |
372 |     builtin_lints: impl EarlyLintPass + 'static + std::marker::Send,
