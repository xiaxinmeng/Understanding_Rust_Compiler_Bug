plain
travis_time:end:0463efb0:start=1545267810836709460,finish=1545267813253883748,duration=2417174288
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    100% |████████████████████████████████| 51kB 10.2MB/s 
Collecting botocore==1.12.69 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/72/ba/a188505f67a78a686aa24d8511a18cb5a8bb27705c9d1b1bb81bee97a138/botocore-1.12.69-py2.py3-none-any.whl (5.2MB)
    0% |                                | 10kB 35.6MB/s eta 0:00:01
    0% |▏                               | 20kB 34.6MB/s eta 0:00:01
    0% |▏                               | 30kB 37.7MB/s eta 0:00:01
    0% |▎                               | 40kB 36.5MB/s eta 0:00:01
---
[00:58:03] .................................................................................................... 2700/5188
[00:58:06] .................................................................................................... 2800/5188
[00:58:09] .................................................................................................... 2900/5188
[00:58:12] .................................................................................................... 3000/5188
[00:58:16] ............................................................................................iF...... 3100/5188
[00:58:22] .......................................................ii..i..ii.................................... 3300/5188
[00:58:26] .................................................................................................... 3400/5188
[00:58:29] .................................................................................................... 3500/5188
[00:58:32] ........................................ii.......................................................... 3600/5188
---
[00:59:24] ---- [ui] ui/macros/macro-comma-behavior.rs#core stdout ----
[00:59:24] diff of stderr:
[00:59:24] 
[00:59:24] 44 
[00:59:24] 45 error: language item required, but not found: `eh_personality`
[00:59:24] 46 
[00:59:24] - error: language item required, but not found: `eh_unwind_resume`
[00:59:24] - error: aborting due to 10 previous errors
[00:59:24] + error: aborting due to 9 previous errors
[00:59:24] 50 
[00:59:24] 51 
[00:59:24] 51 
[00:59:24] 
[00:59:24] 
[00:59:24] The actual stderr differed from the expected stderr.
[00:59:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/macro-comma-behavior.core.stderr
[00:59:24] To update references, rerun the tests and pass the `--bless` flag
[00:59:24] To only update this specific test, also pass `--test-args macros/macro-comma-behavior.rs`
[00:59:24] 
[00:59:24] error in revision `core`: 1 errors occurred comparing output.
[00:59:24] status: exit code: 1
[00:59:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/auxiliary" "-A" "unused"
[00:59:24] ------------------------------------------
[00:59:24] 
[00:59:24] ------------------------------------------
[00:59:24] stderr:
[00:59:24] stderr:
[00:59:24] ------------------------------------------
[00:59:24] {"message":"1 positional argument in format string, but no arguments were given","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-comma-behavior.rs","byte_start":869,"byte_end":871,"line_start":27,"line_end":27,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"    assert_eq!(1, 1, \"{}\",);","highligh`{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# ,","highlight_start":1,"highlight_end":19},{"text":"left_val , right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) => (","highlight_start":1,"highlight_end":77},{"text":"{ assert_eq ! ( $ left , $ right ) } ) ; (","highlight_start":1,"highlight_end":43},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`: {}\"# ,","highlight_start":1,"highlight_end":23},{"text":"left_val , right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: 1 positional argument in format string, but no arguments were given\n  --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:27:23\n   |\nLL |     assert_eq!(1, 1, \"{}\",);\n   |                       ^^\n\n"}
[00:59:24] {"message":"1 positional argument in format string, but no arguments were gitart":1,"highlight_end":30},{"text":"if * left_val == * right_val {","highlight_start":1,"highlight_end":31},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left != right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# ,","highlight_start":1,"highlight_end":19},{"text":"left_val , right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) => {","highlight_start":1,"highlight_end":77},{"text":"assert_ne ! ( $ left , $ right ) } ; (","highlight_start":1,"highlight_end":39},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if * left_val == * right_val {","highlight_start":1,"highlight_end":31},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left != right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`: {}\"# ,","highlight_start":1,"highlight_end":23},{"text":"left_val , right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: 1 positional argument in format stri"text":"    debug_assert_eq!(1, 1, \"{}\",);","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"debug_assert_eq!","def_site_span":{"file_name":"<::core::macros::debug_assert_eq macros>","byte_start":0,"byte_end":95,"line_start":1,"line_end":2,"column_start":1,"column_end":68,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"if cfg ! ( debug_assertions ) { assert_eq ! ( $ ( $ arg ) * ) ; } )","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"assert_eq!","def_site_span":{"file_name":"<::core::macros::assert_eq macros>","byte_start":0,"byte_end":671,"line_start":1,"line_end":21,"column_start":1,"column_end":69,"is_primary":false,"text":[{"text":"( $ left : expr , $ right : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & $ left , & $ right ) {","highlight_start":1,"highlight_end":33},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# ,","highlight_start":1,"highlight_end":19},{"text":"left_val , right_val ) } } } } ) ; ( $ left : expr , $ 39,"column_start":29,"column_end":31,"is_primary":true,"text":[{"text":"    debug_assert_ne!(1, 2, \"{}\",);","highlight_start":29,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<::core::macros::assert_ne macros>","byte_start":610,"byte_end":641,"line_start":21,"line_end":21,"column_start":24,"column_end":55,"is_primary":false,"text":[{"text":"left_val , right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":24,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<::core::macros::debug_assert_ne macros>","byte_start":60,"byte_end":91,"line_start":2,"line_end":2,"column_start":33,"column_end":64,"is_primary":false,"text":[{"text":"if cfg ! ( debug_assertions ) { assert_ne ! ( $ ( $ arg ) * ) ; } )","highlight_start":33,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/macros/macro-comma-behavior.rs","byte_start":1201,"byte_end":1231,"line_start":39,"line_end":39,"column_start":5,"column_end":35,"is_primary":false,"text":[{"text":"    debug_assert_ne!(1, 2, \"{}\",);","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"debug_assert_ne!","def_site_span":{"file_name":"<::core::macros::debug_assert_ne macros>","byte_start":0,"byte_end":95,"line_start":1,"line_end":2,"column_start":1,"column_end":68,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"if cfg ! ( debug_assertions ) { assert_ne ! ( $ ( $ arg ) * ) ; } )","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"assert_ne!","def_site_span":{"file_name":"<::core::macros::assert_ne macros>","byte_start":0,"byte_end":655,"line_start":1,"line_end":21,"column_start":1,"column_end":69,"is_primary":false,"text":[{"text":"( $ left : expr , $ right : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & $ left , & $ right ) {","highlight_start":1,"highlight_end":33},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if * left_val == * right_val {","highlight_start":1,"highlight_end":31},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left != right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# ,","highlight_start":1,"highlight_end":19},{"text":"left_val , right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) => {","highlight_start":1,"highlight_end":77},{"text":"assert_ne ! ( $ left , $ right ) } ; (","highlight_start":1,"highlight_end":39},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {",(\"{}\",);","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: 1 positional argument in format string, but no arguments were given\n  --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:60:19\n   |\nLL |     format_args!(\"{}\",);\n   |                   ^^\n\n"}
[00:59:24] {"message":"1 positional argument in format string, but no arguments were given","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-comma-behavior.rs","byte_start":2168,"byte_end":2170,"line_start":78,"line_end":78,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"    unimplemented!(\"{}\",);","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<::core::macros::unimplemented macros>","byte_start":114,"byte_end":145,"line_start":2,"line_end":2,"column_start":39,"column_end":70,"is_primary":false,"text":[{"text":"panic ! ( \"not yet implemented: {}\" , format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":39,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/macros/macro-comma-behavior.rs","byte_start":2152,"byte_end":2174,"line_start":78,"line_end":78,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    unimplemented!(\"{}\",);","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"s,"highlight_start":21,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/macros/macro-comma-behavior.rs","byte_start":2424,"byte_end":2440,"line_start":87,"line_end":87,"column_start":13,"column_end":29,"is_primary":false,"text":[{"text":"            write!(f, \"{}\",)?;","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"<::core::macros::write macros>","byte_start":0,"byte_end":98,"line_start":1,"line_end":2,"column_start":1,"column_end":56,"is_primary":false,"text":[{"text":"( $ dst : expr , $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":43},{"text":"$ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: 1 positional argument in format string, but no arguments were given\n  --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:87:24\n   |\nLL |             write!(f, \"{}\",)?;\n   |                        ^^\n\n"}
[00:59:24] {"message":"`#[panic_handler]` function required, but not found","code":null,"level":"error","spans":[],"children":[],"rendered":"error: `#[panic_handler]` function required, but not found\n\n"}
[00:59:24] {"message":"language item required, but not found: `eh_personality`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: language item required, but not found: `eh_personality`\n\n"}
[00:59:24] {"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}
[00:59:24] ------------------------------------------
[00:59:24] 
[00:59:24] thread '[ui] ui/macros/macro-comma-behavior.rs#core' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:59:24] 
---
[00:59:24] 
[00:59:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:59:24] 
[00:59:24] 
[00:59:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-testd/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
118652 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115344 ./src/llvm/test/CodeGen
113808 ./obj/build/x86_64-unknown-linux-gnu/stage1
113788 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
---
69912 ./src/llvm-emscripten/lib
68544 ./src/test
62484 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
59088 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
59084 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_6crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ae42a80
travis_time:start:0ae42a80
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d9e5fb5
$ dmesg | grep -i kill
