plain
travis_time:end:07d9602a:start=1552575986006015289,finish=1552576095318560974,duration=109312545685
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
Setting environment variables from .travis.yml
$ export CI_JOB_NAME=$TRAVIS_JOB_NAME
$ export RUST_BACKTRACE=1
$ bash -c 'echo $BASH_VERSION'
4.3.48(1)-release
travis_fold:start:before_install.1
---
[00:01:28]  ---> 3bca462c2690
[00:01:28] Successfully built 3bca462c2690
[00:01:28] Successfully tagged rust-ci:latest
[00:01:28] Built container sha256:3bca462c26901f1ad3b81b1cb45f61990f33aeb959c60f71fd97f691c407cf97
[00:01:28] Uploading finished image to s3://rust-lang-ci-sccache2/docker/8dcfaffd04024de90126ffc81fcdb56fbaa8e5ec2fe8eae50ae68cfe40ef281bfe4085b2d7cde31c578de2a90470847e278efbef6e401173da016643f489dbaa
[00:02:21] upload failed: - to s3://rust-lang-ci-sccache2/docker/8dcfaffd04024de90126ffc81fcdb56fbaa8e5ec2fe8eae50ae68cfe40ef281bfe4085b2d7cde31c578de2a90470847e278efbef6e401173da016643f489dbaa Unable to locate credentials

[00:02:22] travis_time:end:030f5fc4:start=1552576114859565346,finish=1552576246041718467,duration=131182153121
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:02:22] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:12] 
[01:20:12] running 120 tests
[01:20:37] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:20:42] .i......iii.i.....ii
[01:20:42] 
[01:20:42]  finished in 30.672
[01:20:42] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:49:31] 
[01:49:31] running 24 tests
[01:49:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:49:43] ...............F........
[01:49:43] 
[01:49:43] ---- [ui] rustdoc-ui/intra-links-ambiguity.rs stdout ----
[01:49:43] diff of stderr:
[01:49:43] 
[01:49:43] 
[01:49:43] 1 error: `ambiguous` is both a struct and a function
[01:49:43] 2   --> $DIR/intra-links-ambiguity.rs:27:6
[01:49:43] 3    |
[01:49:43] - LL | /// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`
[01:49:43] + LL | /// [`ambiguous`] is ambiguous.
[01:49:43] 6    |
[01:49:43] 7 note: lint level defined here
[01:49:43] 
[01:49:43] 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:49:43] 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:49:43] 12 help: to link to the struct, prefix with the item type
[01:49:43] 13    |
[01:49:43] - LL | /// [`struct@ambiguous`] is ambiguous. //~ERROR `ambiguous`
[01:49:43] + LL | /// [`struct@ambiguous`] is ambiguous.
[01:49:43] 16 help: to link to the function, add parentheses
[01:49:43] 17    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// [`ambiguous()`] is ambiguous. //~ERROR `ambiguous`
[01:49:43] + LL | /// [`ambiguous()`] is ambiguous.
[01:49:43] 20 
[01:49:43] 20 
[01:49:43] 21 error: `ambiguous` is both a struct and a function
[01:49:43] 22   --> $DIR/intra-links-ambiguity.rs:29:6
[01:49:43] 23    |
[01:49:43] 23    |
[01:49:43] - LL | /// [ambiguous] is ambiguous. //~ERROR ambiguous
[01:49:43] + LL | /// [ambiguous] is ambiguous.
[01:49:43] 26 help: to link to the struct, prefix with the item type
[01:49:43] 27    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// [struct@ambiguous] is ambiguous. //~ERROR ambiguous
[01:49:43] + LL | /// [struct@ambiguous] is ambiguous.
[01:49:43] 30 help: to link to the function, add parentheses
[01:49:43] 31    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// [ambiguous()] is ambiguous. //~ERROR ambiguous
[01:49:43] + LL | /// [ambiguous()] is ambiguous.
[01:49:43] 34 
[01:49:43] 34 
[01:49:43] 35 error: `multi_conflict` is a struct, a function, and a macro
[01:49:43] 36   --> $DIR/intra-links-ambiguity.rs:31:6
[01:49:43] 37    |
[01:49:43] 37    |
[01:49:43] - LL | /// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
[01:49:43] + LL | /// [`multi_conflict`] is a three-way conflict.
[01:49:43] 40 help: to link to the struct, prefix with the item type
[01:49:43] 41    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// [`struct@multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
[01:49:43] + LL | /// [`struct@multi_conflict`] is a three-way conflict.
[01:49:43] 44 help: to link to the function, add parentheses
[01:49:43] 45    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// [`multi_conflict()`] is a three-way conflict. //~ERROR `multi_conflict`
[01:49:43] + LL | /// [`multi_conflict()`] is a three-way conflict.
[01:49:43] 48 help: to link to the macro, add an exclamation mark
[01:49:43] 49    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// [`multi_conflict!`] is a three-way conflict. //~ERROR `multi_conflict`
[01:49:43] + LL | /// [`multi_conflict!`] is a three-way conflict.
[01:49:43] 52 
[01:49:43] 52 
[01:49:43] 53 error: `type_and_value` is both a module and a constant
[01:49:43] 54   --> $DIR/intra-links-ambiguity.rs:33:16
[01:49:43] 55    |
[01:49:43] 55    |
[01:49:43] - LL | /// Ambiguous [type_and_value]. //~ERROR type_and_value
[01:49:43] + LL | /// Ambiguous [type_and_value].
[01:49:43] 58 help: to link to the module, prefix with the item type
[01:49:43] 59    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// Ambiguous [module@type_and_value]. //~ERROR type_and_value
[01:49:43] + LL | /// Ambiguous [module@type_and_value].
[01:49:43] 62 help: to link to the constant, prefix with the item type
[01:49:43] 63    |
[01:49:43] 
[01:49:43] 
[01:49:43] - LL | /// Ambiguous [const@type_and_value]. //~ERROR type_and_value
[01:49:43] + LL | /// Ambiguous [const@type_and_value].
[01:49:43] 66 
[01:49:43] 66 
[01:49:43] 67 error: `foo::bar` is both an enum and a function
[01:49:43] 68   --> $DIR/intra-links-ambiguity.rs:35:42
[01:49:43] 69    |
[01:49:43] 69    |
[01:49:43] - LL | /// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`
null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":432,"byte_end":443,"line_start":27,"line_end":27,"column_start":6,"column_end":17,"is_primary":true,"text":[{"text":"/// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`","highlight_start":6,"highlight_end":17}],"label":null,"suggested_replacement":"`struct@ambiguous`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"to link to the function, add parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":432,"byte_end":443,"line_start":27,"line_end":27,"column_start":6,"column_end":17,"is_primary":true,"text":[{"text":"/// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`","highlight_start":6,"highlight_end":17}],"label":null,"suggested_replacement":"`ambiguous()`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `ambiguous` is both a struct and a function\n  --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:27:6\n   |\nLL | /// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`\n   |      ^^^^^^^^^^^ ambiguous link\n   |\nnote: lint level defined here\n  --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:1:9\n   |\nLL | #![deny(intra_doc_link_resolution_failure)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nhelp: to link to the struct, prefix with the item type\n   |\nLL | /// [`struct@ambiguous`] is ambiguous. //~ERROR `ambiguous`\n   |      ^^^^^^^^^^^^^^^^^^\nhelp: to link to the function, add parentheses\n   |\nLL | /// [`ambiguous()`] is ambiguous. //~ERROR `ambiguous`\n   |      ^^^^^^^^^^^^^\n\n"}
[01:49:43] {"message":"`ambiguous` is both a struct and a function","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":489,"byte_end":498,"line_start":29,"line_end":29,"column_start":6,"column_end":15,"is_primary":true,"text":[{"text":"/// [ambiguous] is ambiguous. //~ERROR ambiguous","highlight_start":6,"highlight_end":15}],"label":"ambiguous link","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to link to the struct, prefix with the item type","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":489,"byte_end":498,"line_start":29,"line_end":29,"column_start":6,"column_end":15,"is_primary":true,"text":[{"text":"/// [ambiguous] is ambiguous. //~ERROR ambiguous","highlight_start":6,"highlight_end":15}],"label":null,"suggested_replacement":"struct@ambiguous","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"to link to the function, add parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":489,"byte_end":498,"line_start":29,"line_end":29,"column_start":6,"column_end":15,"is_primary":true,"text":[{"text":"/// [ambiguous] is ambiguous. //~ERROR ambiguous","highlight_start":6,"highlight_end":15}],"label":null,"suggested_replacement":"ambiguous()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `ambiguous` is both a struct and a function\n  --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:29:6\n   |\nLL | /// [ambiguous] is ambiguous. //~ERROR ambiguous\n   |      ^^^^^^^^^ ambiguous link\nhelp: to link to the struct, prefix with the item type\n   |\nLL | /// [struct@ambiguous] is ambiguous. //~ERROR ambiguous\n   |      ^^^^^^^^^^^^^^^^\nhelp: to link to the function, add parentheses\n   |\nLL | /// [ambiguous()] is ambiguous. //~ERROR ambiguous\n   |      ^^^^^^^^^^^\n\n"}
[01:49:43] {"message":"`multi_conflict` is a struct, a function, and a macro","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":542,"byte_end":558,"line_start":31,"line_end":31,"column_start":6,"column_end":22,"is_primary":true,"text":[{"text":"/// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`","highlight_start":6,"highlight_end":22}],"label":"ambiguous link","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to link to the struct, prefix with the item type","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":542,"byte_end":558,"line_start":31,"line_end":31,"column_start":6,"column_end":22,"is_primary":true,"text":[{"text":"/// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`","highlight_start":6,"highlight_end":22}],"label":null,"suggested_replacement":"`struct@multi_conflict`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"to link to the function, add parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":542,"byte_end":558,"line_start":31,"line_end":31,"column_start":6,"column_end":22,"is_primary":true,"text":[{"text":"/// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`","highlight_start":6,"highlight_end":22}],"label":null,"suggested_replacement":"`multi_conflict()`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"to link to the macro, add an exclamation mark","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":542,"byte_end":558,"line_start":31,"line_end":31,"column_start":6,"column_end":22,"is_primary":true,"text":[{"text":"/// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`","highlight_start":6,"highlight_end":22}],"label":null,"suggested_replacement":"`multi_conflict!`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `multi_conflict` is a struct, a function, and a macro\n  --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:31:6\n   |\nLL | /// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`\n   |      ^^^^^^^^^^^^^^^^ ambiguous link\nhelp: to link to the struct, prefix with the item type\n   |\nLL | /// [`struct@multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`\n   |      ^^^^^^^^^^^^^^^^^^^^^^^\nhelp: to link to the function, add parentheses\n   |\nLL | /// [`multi_conflict()`] is a three-way conflict. //~ERROR `multi_conflict`\n   |      ^^^^^^^^^^^^^^^^^^\nhelp: to link to the macro, add an exclamation mark\n   |\nLL | /// [`multi_conflict!`] is a three-way conflict. //~ERROR `multi_conflict`\n   |      ^^^^^^^^^^^^^^^^^\n\n"}
[01:49:43] {"message":"`type_and_value` is both a module and a constant","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":630,"byte_end":644,"line_start":33,"line_end":33,"column_start":16,"column_end":30,"is_primary":true,"text":[{"text":"/// Ambiguous [type_and_value]. //~ERROR type_and_value","highlight_start":16,"highlight_end":30}],"label":"ambiguous link","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to link to the module, prefix with the item type","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":630,"byte_end":644,"line_start":33,"line_end":33,"column_start":16,"column_end":30,"is_primary":true,"text":[{"text":"/// Ambiguous [type_and_value]. //~ERROR type_and_value","highlight_start":16,"highlight_end":30}],"label":null,"suggested_replacement":"module@type_and_value","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"to link to the constant, prefix with the item type","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":630,"byte_end":644,"line_start":33,"line_end":33,"column_start":16,"column_end":30,"is_primary":true,"text":[{"text":"/// Ambiguous [type_and_value]. //~ERROR type_and_value","highlight_start":16,"highlight_end":30}],"label":null,"suggested_replacement":"const@type_and_value","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `type_and_value` is both a module and a constant\n  --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:33:16\n   |\nLL | /// Ambiguous [type_and_value]. //~ERROR type_and_value\n   |                ^^^^^^^^^^^^^^ ambiguous link\nhelp: to link to the module, prefix with the item type\n   |\nLL | /// Ambiguous [module@type_and_value]. //~ERROR type_and_value\n   |                ^^^^^^^^^^^^^^^^^^^^^\nhelp: to link to the constant, prefix with the item type\n   |\nLL | /// Ambiguous [const@type_and_value]. //~ERROR type_and_value\n   |                ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:49:43] {"message":"`foo::bar` is both an enum and a function","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":716,"byte_end":726,"line_start":35,"line_end":35,"column_start":42,"column_end":52,"is_primary":true,"text":[{"text":"/// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`","highlight_start":42,"highlight_end":52}],"label":"ambiguous link","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to link to the enum, prefix with the item type","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":716,"byte_end":726,"line_start":35,"line_end":35,"column_start":42,"column_end":52,"is_primary":true,"text":[{"text":"/// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`","highlight_start":42,"highlight_end":52}],"label":null,"suggested_replacement":"`enum@foo::bar`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"to link to the function, add parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs","byte_start":716,"byte_end":726,"line_start":35,"line_end":35,"column_start":42,"column_end":52,"is_primary":true,"text":[{"text":"/// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`","highlight_start":42,"highlight_end":52}],"label":null,"suggested_replacement":"`foo::bar()`","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `foo::bar` is both an enum and a function\n  --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:35:42\n   |\nLL | /// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`\n   |                                          ^^^^^^^^^^ ambiguous link\nhelp: to link to the enum, prefix with the item type\n   |\nLL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`]. //~ERROR `foo::bar`\n   |                                          ^^^^^^^^^^^^^^^\nhelp: to link to the function, add parentheses\n   |\nLL | /// Ambiguous non-implied shortcut link [`foo::bar()`]. //~ERROR `foo::bar`\n   |                                          ^^^^^^^^^^^^\n\n"}
[01:49:43] 
[01:49:43] ------------------------------------------
[01:49:43] 
[01:49:43] thread '[ui] rustdoc-ui/intra-links-ambiguity.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:49:43] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:49:43] 
[01:49:43] 
[01:49:43] 
[01:49:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/travis_time:end:1a07d32e:start=1552576103979823571,finish=1552582687734522324,duration=6583754698753
travis_time:start:1789cc65
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 14 16:58:07 UTC 2019
Thu, 14 Mar 2019 16:58:08 GMT
---
travis_time:end:138e56d4:start=1552582689217447664,finish=1552582689222181337,duration=4733673
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:000fe8a8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:136d25d0
travis_time:start:136d25d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:053609a8
$ dmesg | grep -i kill
