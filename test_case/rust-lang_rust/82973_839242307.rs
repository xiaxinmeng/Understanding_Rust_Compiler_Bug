plain
[RUSTC-TIMING] gimli test:false 5.230
[RUSTC-TIMING] object test:false 11.183
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error[E0412]: cannot find type `ExitStatusError` in module `imp`
     |
     |
1531 | pub struct ExitStatusError(imp::ExitStatusError);
     |                                 ^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `ExitStatus`
     | 
    ::: library/std/src/sys/wasm/../unsupported/process.rs:97:1
     |
97   | pub struct ExitStatus(!);
     | ------------------------- similarly named struct `ExitStatus` defined here

error[E0599]: no method named `exit_ok` found for struct `wasm::process::ExitStatus` in the current scope
     |
     |
1425 |         self.0.exit_ok().map_err(ExitStatusError)
     |                ^^^^^^^ method not found in `wasm::process::ExitStatus`
     | 
    ::: library/std/src/sys/wasm/../unsupported/process.rs:97:1
     |
97   | pub struct ExitStatus(!);
     | ------------------------- method `exit_ok` not found for this

error[E0599]: no method named `exit_ok` found for struct `wasm::process::ExitStatus` in the current scope
     |
     |
1449 |         self.0.exit_ok().is_ok()
     |                ^^^^^^^ method not found in `wasm::process::ExitStatus`
     | 
    ::: library/std/src/sys/wasm/../unsupported/process.rs:97:1
     |
97   | pub struct ExitStatus(!);
     | ------------------------- method `exit_ok` not found for this
error: aborting due to 3 previous errors; 1 warning emitted

Some errors have detailed explanations: E0412, E0599.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
[RUSTC-TIMING] std test:false 1.572
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/mir-opt src/test/codegen-units library/core
Build completed unsuccessfully in 0:14:52
