plain
[01:26:58] 
[01:26:58] doc tests for: /checkout/src/doc/reference/src/abi.md
[01:26:58] 
[01:26:58] running 3 tests
[01:26:58] test /checkout/src/doc/reference/src/abi.md - Application_Binary_Interface__ABI_::The_ (line 71) ... ignored
[01:26:58] test /checkout/src/doc/reference/src/abi.md - Application_Binary_Interface__ABI_::The_ (line 83) ... ignored
[01:26:59] test /checkout/src/doc/reference/src/abi.md - Application_Binary_Interface__ABI_::The_ (line 19) ... ok
[01:26:59] test result: ok. 1 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
[01:26:59] 
[01:26:59] doc tests for: /checkout/src/doc/reference/src/attributes/codegen.md
[01:26:59] 
---
[01:27:15] 
[01:27:15] doc tests for: /checkout/src/doc/reference/src/items/external-blocks.md
[01:27:16] 
[01:27:16] running 4 tests
[01:27:16] test /checkout/src/doc/reference/src/items/external-blocks.md - External_blocks::ABI (line 60) ... ignored
[01:27:16] test /checkout/src/doc/reference/src/items/external-blocks.md - External_blocks::Attributes_on_extern_blocks::The_ (line 124) ... ignored
[01:27:16] test /checkout/src/doc/reference/src/items/external-blocks.md - External_blocks::Attributes_on_extern_blocks::The_ (line 147) ... ignored
[01:27:16] test /checkout/src/doc/reference/src/items/external-blocks.md - External_blocks::Variadic_functions (line 100) ... ignored
[01:27:16] test result: ok. 0 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out
[01:27:16] 
[01:27:16] doc tests for: /checkout/src/doc/reference/src/items/functions.md
[01:27:16] 
---
[01:27:22] 
[01:27:22] doc tests for: /checkout/src/doc/reference/src/macro-ambiguity.md
[01:27:22] 
[01:27:22] running 1 test
[01:27:22] test /checkout/src/doc/reference/src/macro-ambiguity.md - Appendix__Macro_Follow_Set_Ambiguity_Formal_Specification::Definitions_ (line 43) ... ok
[01:27:22] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:27:22] 
[01:27:22] doc tests for: /checkout/src/doc/reference/src/macros-by-example.md
[01:27:22] 
[01:27:22] 
[01:27:22] running 14 tests
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Hygiene (line 358) ... ignored
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing (line 202) ... ignored
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Hygiene (line 423) ... ok
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 223) ... ignored
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Path_Based_Scope (line 330) ... ok
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Hygiene (line 400) ... ok
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Hygiene (line 383) ... ok
[01:27:22] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 313) ... ignored
[01:27:23] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 274) ... ok
[01:27:23] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 246) ... ok
[01:27:23] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Transcribing (line 63) ... ok
[01:27:23] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Transcribing (line 102) ... ok
[01:27:23] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 293) ... ok
[01:27:23] test /checkout/src/doc/reference/src/macros-by-example.md - Macros_By_Example::Transcribing (line 86) ... ok
[01:27:23] test result: ok. 10 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out
[01:27:23] 
[01:27:23] doc tests for: /checkout/src/doc/reference/src/macros.md
[01:27:23] 
---
[01:48:39] Verifying status of miri...
[01:48:39] Verifying status of embedded-book...
[01:48:39] This PR updated 'src/doc/embedded-book', verifying if status is 'test-pass'...
[01:48:39] 
[01:48:39] ⚠️ We detected that this PR updated 'embedded-book', but its tests failed.
[01:48:39] 
[01:48:39] If you do intend to update 'embedded-book', please check the error messages above and
[01:48:39] commit another update.
[01:48:39] 
[01:48:39] If you do NOT intend to update 'embedded-book', please ensure you did not accidentally
[01:48:39] change the submodule at 'src/doc/embedded-book'. You may ask your reviewer for the
[01:48:39] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:125ec0fb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 24 14:25:05 UTC 2019
---
travis_time:end:0dcc41de:start=1553437507824594143,finish=1553437507841116268,duration=16522125
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0211b3db
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0eb9e200
travis_time:start:0eb9e200
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:261ea9d8
$ dmesg | grep -i kill
