plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/25/f5/97c93a73bae1d988facef411e5b81d809fb9f7eba26cb3c09b65d0ca5077/awscli-1.15.32-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 5.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.8MB/s eta 0:00:01
---
    100% |████████████████████████████████| 1.3MB 990kB/s 
Collecting botocore==1.10.32 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b5/33/4b424070c1a06da89dc8c04d74c2b07cadc038d652c89b31121f5924acb7/botocore-1.10.32-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 40.8MB/s eta 0:00:01
    0% |▏                               | 20kB 40.4MB/s eta 0:00:01
    0% |▎                               | 30kB 47.6MB/s eta 0:00:01
    0% |▎                               | 40kB 35.7MB/s eta 0:00:01
---
[00:44:22] .............................................................................i......................
[00:44:27] ....................................................................................................
[00:44:33] ....................................................................................................
[00:44:39] ....................................................................................................
[00:44:44] ..........i..................iiiiiiiii.F.FF................................................
[00:44:44] 
[00:44:44] ---- [ui] ui/tuple-struct-fields/test2.rs stdout ----
[00:44:44] normalized stderr:
[00:44:44] normalized stderr:
[00:44:44] error: expected one of `)` or `,`, found `(`
[00:44:44]    |
[00:44:44]    |
[00:44:44] LL |         struct S3(pub $t ());
[00:44:44]    |                          ^ expected one of `)` or `,` here
[00:44:44] ...
[00:44:44] LL |     define_struct! { (foo) }
[00:44:44] 
[00:44:44] error: aborting due to previous error
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] The actual stderr differed from the expected stderr.
[00:44:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test2/test2.stderr
[00:44:44] To update references, rerun the tests and pass the `--bless` flag
[00:44:44] To only update this specific test, also pass `--test-args tuple-struct-fields/test2.rs`
[00:44:44] error: 1 errors occurred comparing output.
[00:44:44] status: exit code: 101
[00:44:44] status: exit code: 101
[00:44:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tuple-struct-fields/test2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test2/auxiliary" "-A" "unused"
[00:44:44] ------------------------------------------
[00:44:44] 
[00:44:44] ------------------------------------------
[00:44:44] stderr:
[00:44:44] stderr:
[00:44:44] ------------------------------------------
[00:44:44] {"message":"expected one of `)` or `,`, found `(`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/tuple-struct-fields/test2.rs","byte_start":601,"byte_end":602,"line_start":15,"line_end":15,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"        struct S3(pub $t ());","highlight_start":26,"highlight_end":27}],"label":"expected one of `)` or `,` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/tuple-struct-fields/test2.rs","byte_start":686,"byte_end":710,"line_start":21,"line_end":21,"column_start":5,"column_end":29,"is_primary":false,"text":[{"text":"    define_struct! { (foo) }","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"define_struct!","def_site_span":{"file_name":"/checkout/src/test/ui/tuple-struct-fields/test2.rs","byte_start":467,"byte_end":670,"line_start":11,"line_end":18,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! define_struct {","highlight_start":1,"highlight_end":29},{"text":"    ($t:ty) => {","highlight_start":1,"highlight_end":17},{"text":"        struct S1(pub $t);","highlight_start":1,"highlight_end":27},{"text":"        struct S2(pub (in foo) ());","highlight_start":1,"highlight_end":36},{"text":"        struct S3(pub $t ());","highlight_start":1,"highlight_end":30},{"text":"        //~^ ERROR expected one of `)` or `,`, found `(`","highlight_start":1,"highlight_end":57},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: expected one of `)` or `,`, found `(`\n  --> /checkout/src/test/ui/tuple-struct-fields/test2.rs:15:26\n   |\nLL |         struct S3(pub $t ());\n   |                          ^ expected one of `)` or `,` here\n...\nLL |     define_struct! { (foo) }\n   |     ------------------------ in this macro invocation\n\n"}
[00:44:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:44] ------------------------------------------
[00:44:44] 
[00:44:44] thread '[ui] ui/tuple-struct-fields/test2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:44:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:44] 
[00:44:44] ---- [ui] ui/tuple-struct-fields/test.rs stdout ----
[00:44:44] normalized stderr:
[00:44:44] error: expected one of `)` or `,`, found `(`
[00:44:44]    |
[00:44:44]    |
[00:44:44] LL |     struct S2(pub((foo)) ());
[00:44:44]    |                          ^ expected one of `)` or `,` here
[00:44:44] 
[00:44:44] error[E0412]: cannot find type `foo` in this scope
[00:44:44]    |
[00:44:44]    |
[00:44:44] LL |     struct S2(pub((foo)) ());
[00:44:44] 
[00:44:44] 
[00:44:44] error[E0601]: `main` function not found in crate `test`
[00:44:44]    |
[00:44:44]    = note: consider adding a `main` function to `$DIR/test.rs`
[00:44:44] error: aborting due to 3 previous errors
[00:44:44] 
[00:44:44] Some errors occurred: E0412, E0601.
[00:44:44] For more information about an error, try `rustc --explain E0412`.
[00:44:44] For more information about an error, try `rustc --explain E0412`.
[00:44:44] 
[00:44:44] 
[00:44:44] 
[00:44:44] The actual stderr differed from the expected stderr.
[00:44:44] 581,"byte_end":584,"line_start":14,"line_end":14,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    struct S2(pub((foo)) ());","highlight_start":20,"highlight_end":23}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0412]: cannot find type `foo` in this scope\n  --> /checkout/src/test/ui/tuple-struct-fields/test.rs:14:20\n   |\nLL |     struct S2(pub((foo)) ());\n   |                    ^^^ not found in this scope\n\n"}
[00:44:44] {"message":"`main` function not found in crate `test`","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n