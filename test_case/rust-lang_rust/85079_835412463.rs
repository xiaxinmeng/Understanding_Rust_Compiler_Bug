plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved imports `rustc_session::config::build_codegen_options`, `rustc_session::config::build_debugging_options`
   |
   |
11 |     build_codegen_options, build_debugging_options, get_cmd_lint_options, host_triple,
   |     ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^ no `build_debugging_options` in `config`
   |     no `build_codegen_options` in `config`
   |
help: a similar name exists in the module
   |
   |
11 |     build_session_options, build_debugging_options, get_cmd_lint_options, host_triple,
help: a similar name exists in the module
   |
   |
11 |     build_codegen_options, build_session_options, get_cmd_lint_options, host_triple,

error[E0425]: cannot find function `basic_debugging_options` in module `config`
  --> src/librustdoc/doctest.rs:79:62
   |
   |
79 |         debugging_opts: config::DebuggingOptions { ..config::basic_debugging_options() },

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0432.
