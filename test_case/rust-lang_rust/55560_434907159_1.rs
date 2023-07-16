\n\nPlease be sure that a file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/modules/mod_dir_path3.rs","byte_start":569,"byte_end":573,"line_start":16,"line_end":16,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    pub mod test;","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either pancakes/test.rs or pancakes/test/mod.rs inside the directory \"/checkout/src/test/run-pass/modules\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `test`\n  --> /checkout/src/test/run-pass/modules/mod_dir_path3.rs:16:13\n   |\nLL |     pub mod test;\n   |             ^^^^\n   |\n   = help: name the file either pancakes/test.rs or pancakes/test/mod.rs inside the directory \"/checkout/src/test/run-pass/modules\"\n\n"}
[00:55:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:55:31] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[00:55:31] ------------------------------------------
[00:55:31] 
[00:55:31] thread '[run-pass] run-pass/modules/mod_dir_path3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:55:31] 
---

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ca4b150
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov  1 02:08:17 UTC 2018
Thu, 01 Nov 2018 02:08:17 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
