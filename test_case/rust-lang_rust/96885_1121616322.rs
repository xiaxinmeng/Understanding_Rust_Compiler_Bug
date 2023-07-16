
error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module
  --> src/timer/posix.rs:72:39
   |
72 |     #[link(name = "os-timer-posix-c", lind = "static")]
   |                                       ^^^^^^^^^^^^^^^
