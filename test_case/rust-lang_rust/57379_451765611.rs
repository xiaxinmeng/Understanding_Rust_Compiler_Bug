plain
travis_time:end:0b82c950:start=1546797525077053598,finish=1546797526224509124,duration=1147455526
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:34] .................................................................................................... 3600/5298
[01:01:38] ......................................ii............................................................ 3700/5298
[01:01:40] ........................................................i........................................... 3800/5298
[01:01:42] .................................................................................................... 3900/5298
[01:01:45] ............i.F..................................................................................... 4000/5298
[01:02:00] .................................................................................................... 4200/5298
[01:02:04] .................................................................................................... 4300/5298
[01:02:08] .................................................................................................... 4400/5298
[01:02:12] .......................................................i............................................ 4500/5298
---
[01:02:44] 
[01:02:44] ---- [ui] ui/parser/mod_file_not_exist.rs stdout ----
[01:02:44] diff of stderr:
[01:02:44] 
[01:02:44] 1 error[E0583]: file not found for module `not_a_real_file`
[01:02:44] +   --> $DIR/mod_file_not_exist.rs:3:5
[01:02:44] 3    |
[01:02:44] 3    |
[01:02:44] 4 LL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`
[01:02:44] 5    |     ^^^^^^^^^^^^^ing to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/mod_file_not_exist.rs","byte_start":23,"byte_end":38,"line_start":3,"line_end":3,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory \"/checkout/src/test/ui/parser\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `not_a_real_file`\n  --> /checkout/src/test/ui/parser/mod_file_not_exist.rs:3:5\n   |\nLL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory \"/checkout/src/test/ui/parser\"\n\n"}
[01:02:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:02:44] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[01:02:44
