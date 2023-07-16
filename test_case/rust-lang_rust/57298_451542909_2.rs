\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/run-pass/used_underscore_binding_macro.rs","byte_start":543,"byte_end":569,"line_start":14,"line_end":14,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate serde_derive;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `serde_derive`\n  --> tests/run-pass/used_underscore_binding_macro.rs:14:1\n   |\nLL | extern crate serde_derive;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
{"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
{"message":"Some errors occurred: E0463, E0464.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0463, E0464.\n"}
{"message":"For more information about an error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0463`.\n"}

------------------------------------------

thread '[run-pass] run-pass/used_underscore_binding_macro.rs' panicked at 'explicit panic', /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.18/src/runtest.rs:2632:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    [run-pass] run-pass/used_underscore_binding_macro.rs

test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

test compile_test ... FAILED

failures:

---- compile_test stdout ----
thread 'compile_test' panicked at 'Some tests failed', /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.18/src/lib.rs:90:22


failures:
    compile_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test compile-test'

