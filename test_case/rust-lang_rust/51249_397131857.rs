plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e2/29/f5b58658602baf037db0f650567d95e8d36104e1bd1966fa4628d7c7e470/awscli-1.15.38-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 7.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
[00:42:29] ....................................................................................................
[00:42:34] ....................................................................................................
[00:42:40] ....................................................................................................
[00:42:45] ....i..............................................................................i................
[00:42:50] ...................................................................................................F
[00:42:55] F...................................................................................................
[00:43:01] ........................................................................................F...........
[00:43:07] ...
[00:43:07] failures:
[00:43:07] 
[00:43:07] ---- [ui] ui/rfc-2005-default-binding-mode/enum.rs stdout ----
[00:43:07] ---- [ui] ui/rfc-2005-default-binding-mode/enum.rs stdout ----
[00:43:07] diff of stderr:
[00:43:07] 
[00:43:07] 2   --> $DIR/enum.rs:19:5
[00:43:07] 3    |
[00:43:07] 4 LL |     let Wrap(x) = &Wrap(3);
[00:43:07] -    |              - consider changing this to `x`
[00:43:07] +    |              - help: use a mutable reference instead: um.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/auxiliary" "-A" "unused"
[00:43:07] ------------------------------------------
[00:43:07] 
[00:43:07] ------------------------------------------
[00:43:07] stderr:
[00:43:07] stderr:
[00:43:07] ------------------------------------------
[00:43:07] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":568,"byte_end":575,"line_start":19,"line_end":19,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    *x += 1; //~ ERROR cannot assign to immutable","highlight_start":5,"highlight_end":12}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a mutable reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":549,"byte_end":550,"line_start":18,"line_end":18,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let Wrap(x) = &Wrap(3);","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"Unspe] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:43:07] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:43:07] ------------------------------------------
[00:43:07] 
[00:43:07] thread '[ui] ui/rfc-2005-default-binding-mode/enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:07] 
[00:43:07] ---- [ui] ui/rfc-2005-default-binding-mode/explicit-mut.rs stdout ----
[00:43:07] diff of stderr:
[00:43:07] 
[00:43:07] 2   --> $DIR/explicit-mut.rs:17:13
[00:43:07] 3    |
[00:43:07] 4 LL |         Some(n) => {
[00:43:07] -    |              - consider changing this to `n`
[00:43:07] +    |              - help: use a mutable reference instead: `n`
[00:43:07] 6 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:43:07] 7    |             ^^^^^^^ cannot borrow as mutable
[00:43:07] 
[00:43:07] 10   --> $DIR/explicit-mut.rs:25:13
[00:43:07] 11    |
[00:43:07] 11    |
[00:43:07] 12 LL |         Some(n) => {
[00:43:07] -    |              - consider changing this to `n`
[00:43:07] +    |              - help: use a mutable reference instead: `n`
[00:43:07] 14 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:43:07] 15    |             ^^^^^^^ cannot borrow as mutable
[00:43:07] 
[00:43:07] 18   --> $DIR/explicit-mut.rs:33:13
[00:43:07] 19    |
[00:43:07] 19    |
[00:43:07] 20 LL |         Some(n) => {
[00:43:07] -    |              - consider changing this to `n`
[00:43:07] +    |              - help: use a mutable reference instead: `n`
[00:43:07] 22 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:43:07] 23    |             ^^^^^^^ cannot borrow as mutable
[00:43:07] 
[00:43:07] 
[00:43:07] The actual stderr differed from the expected stderr.
[00:43:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[00:43:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[00:43:07] To update references, rerun the tests and pass the `--bless` flag
[00:43:07] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/explicit-mut.rs`
[00:43:07] error: 1 errors occurred comparing output.
[00:43:07] status: exit code: 101
[00:43:07] status: exit code: 101
[00:43:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/auxiliary" "-A" "unused"
[00:43:07] ------------------------------------------
[00:43:07] 
[00:43:07] 
[00:43:07] --------------0:43:07] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":828,"byte_end":835,"line_start":25,"line_end":25,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a mutable reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":808,"byte_end":809,"line_start":24,"line_end":24,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:25:13\n   |\nLL |         Some(n) => {\n   |              - help: use a mutable reference instead: `n`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:43:07] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":997,"byte_end":1004,"line_start":33,"line_end":33,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a mutable reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":977,"byte_end":978,"line_start":32,"line_end":32,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:33:13\n   |\nLL |         Some(n) => {\n   |              - help: use a mutable reference instead: `n`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:43:07] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:43:07] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:43:07] -Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/auxiliary" "-A" "unused"
[00:43:07] ------------------------------------------
[00:43:07] 
[00:43:07] ------------------------------------------
[00:43:07] stderr:
[00:43:07] stderr:
[00:43:07] ------------------------------------------
[00:43:07] {"message":"cannot assign to immutable borrowed content `*my_ref`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":511,"byte_end":522,"line_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]","highlight_start":5,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a mutable reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":487,"byte_end":501,"line_start":12,"line_end":12,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"    let ref my_ref @ _ = 0;","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":"ref mut my_ref @ _","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*my_ref`\n  --> /checkout/src/test/ui/suggestions/issue-51244.rs:13:5\n   |\nLL |     let ref my_ref @ _ = 0;\n   |         -------------- help: use a mutable reference instead: `ref mut my_ref @ _`\nLL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]\n   |     ^^^^^^^^^^^ cannot borrow as mutable\n\n"}
[00:43:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:07] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:43:07] ------------------------------------------
[00:43:07] 
[00:43:07] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:43:07] 
---
[00:43:07] test result: FAILED. 1495 passed; 3 failed; 5 ignored; 0 measured; 0 filtered out
[00:43:07] 
[00:43:07] 
[00:43:07] 
[00:43:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stag
60840 ./src/llvm-emscripten/lib
59904 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
56620 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
56616 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
