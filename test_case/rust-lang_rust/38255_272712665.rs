
---- [compile-fail] compile-fail/invalid-module-declaration.rs stdout ----

error: auxiliary build of "/Users/imperio/rust/rust/src/test/compile-fail/auxiliary/module-suggestion.rs" failed to compile:
status: exit code: 101
command: x86_64-apple-darwin/stage1/bin/rustc /Users/imperio/rust/rust/src/test/compile-fail/auxiliary/module-suggestion.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin --error-format json --crate-type=dylib -L x86_64-apple-darwin/test/compile-fail/invalid-module-declaration.stage1-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic --out-dir x86_64-apple-darwin/test/compile-fail/invalid-module-declaration.stage1-x86_64-apple-darwin.compile-fail.libaux --cfg rtopt -C rpath -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{"message":"cannot declare a new module at this location","code":null,"level":"error","spans":[{"file_name":"/Users/imperio/rust/rust/src/test/compile-fail/auxiliary/foo/bar.rs","byte_start":1452,"byte_end":1455,"line_start":11,"line_end":11,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"pub mod baz;","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"maybe move this module `/Users/imperio/rust/rust/src/test/compile-fail/auxiliary/foo/bar/mod.rs` to its own directory via `/Users/imperio/rust/rust/src/test/compile-fail/auxiliary/foo/bar/mod.rs`","code":null,"level":"note","spans":[{"file_name":"/Users/imperio/rust/rust/src/test/compile-fail/auxiliary/foo/bar.rs","byte_start":1452,"byte_end":1455,"line_start":11,"line_end":11,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"pub mod baz;","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":null}
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":null}

------------------------------------------

thread '[compile-fail] compile-fail/invalid-module-declaration.rs' panicked at 'explicit panic', /Users/imperio/rust/rust/src/tools/compiletest/src/runtest.rs:2465
