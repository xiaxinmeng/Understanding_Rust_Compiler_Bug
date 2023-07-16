\n\nMust always be called with exactly two arguments, e.g. `f(2, \"test\")`.\n\nNote that Rust does not have a notion of optional function arguments or\nvariadic functions (except for its C-FFI).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail-fulldeps/auxiliary/lint_group_plugin_test.rs","byte_start":1645,"byte_end":1664,"line_start":52,"line_end":52,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"    reg.register_lint_group(\"lint_me\", vec![TEST_LINT, PLEASE_LINT]);","highlight_start":9,"highlight_end":28}],"label":"expected 3 parameters","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0061]: this function takes 3 parameters but 2 parameters were supplied\n  --> /checkout/src/test/compile-fail-fulldeps/auxiliary/lint_group_plugin_test.rs:52:9\n   |\nLL |     reg.register_lint_group(\"lint_me\", vec![TEST_LINT, PLEASE_LINT]);\n   |         ^^^^^^^^^^^^^^^^^^^ expected 3 parameters\n\n"}
[01:02:13] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:02:13] {"message":"For more information about this error, try `rustc --explain E0061`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0061`.\n"}
[01:02:13] ------------------------------------------
[01:02:13] 
[01:02:13] thread '[compile-fail] compile-fail-fulldeps/lint-group-plugin-deny-cmdline.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:02:13] thread '[compile-fail] compile-fail-fulldeps/lint-group-plugin-deny-cmdline.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5

