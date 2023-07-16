plain
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...................................i...ii................
failures:

---- [run-make] src/test/run-make-fulldeps/issue-19371 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
--- stderr -------------------------------
--- stderr -------------------------------
error[E0560]: struct `Config` has no field named `diagnostic_output`
   |
58 |         diagnostic_output: DiagnosticOutput::Default,
   |         ^^^^^^^^^^^^^^^^^ `Config` does not have this field
   |
   |
   = note: available fields are: `opts`, `crate_cfg`, `crate_check_cfg`, `input`, `input_path` ... and 9 others
error: aborting due to previous error

For more information about this error, try `rustc --explain E0560`.
make: *** [Makefile:8: all] Error 1
