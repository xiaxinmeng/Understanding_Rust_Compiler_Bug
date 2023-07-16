plain
Caused by:
  could not parse input as TOML

Caused by:
  TOML parse error at line 33, column 1
     |
  33 | rustc_symbol_mangling = { path = "../rustc_symbol_mangling" }
     | ^
  Duplicate key `rustc_symbol_mangling` in table `dependencies`
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:00:17
