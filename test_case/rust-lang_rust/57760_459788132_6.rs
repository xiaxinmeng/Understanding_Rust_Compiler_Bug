\n\nCertain Rust types must be cast before passing them to a variadic function,\nbecause of arcane ABI rules dictated by the C standard. To fix the error,\ncast the value to the type specified by the error message (which you may need\nto import from `std::os::raw`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0617.rs","byte_start":849,"byte_end":855,"line_start":24,"line_end":24,"column_start":36,"column_end":42,"is_primary":true,"text":[{"text":"        printf(::std::ptr::null(), printf);","highlight_start":36,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"cast the value to `for<'r> unsafe extern \"C\" fn(*const i8, std::ffi::VaList<'r>, ...)`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0617.rs","byte_start":849,"byte_end":855,"line_start":24,"line_end":24,"column_start":36,"column_end":42,"is_primary":true,"text":[{"text":"        printf(::std::ptr::null(), printf);","highlight_start":36,"highlight_end":42}],"label":null,"suggested_replacement":"printf as for<'r> unsafe extern \"C\" fn(*const i8, std::ffi::VaList<'r>, ...)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0617]: can't pass `for<'r> unsafe extern \"C\" fn(*const i8, std::ffi::VaList<'r>, ...) {printf}` to variadic function\n  --> /checkout/src/test/ui/error-codes/E0617.rs:24:36\n   |\nLL |         printf(::std::ptr::null(), printf);\n   |                                    ^^^^^^\nhelp: cast the value to `for<'r> unsafe extern \"C\" fn(*const i8, std::ffi::VaList<'r>, ...)`\n   |\nLL |         printf(::std::ptr::null(), printf as for<'r> unsafe extern \"C\" fn(*const i8, std::ffi::VaList<'r>, ...));\n   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:02:39] {"message":"For more information about this error, try `rustc --explain E0617`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0617`.\n"}
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] 
[01:02:39] thread '[ui] ui/error-codes/E0617.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:02:39] 
[01:02:39] ---- [ui] ui/invalid/invalid-variadic-function.rs stdout ----
[01:02:39] diff of stderr:
[01:02:39] 
[01:02:39] - error: only foreign functions are allowed to be variadic
[01:02:39] + error: expected argument name, found `...`
[01:02:39] 3    |
[01:02:39] 3    |
[01:02:39] 4 LL | extern "C" fn foo(x: u8, ...);
[01:02:39] -    |                          ^^^
[01:02:39] +    |                          ^^^ expected argument name
[01:02:39] 6 
[01:02:39] 6 
[01:02:39] 7 error: expected one of `->`, `where`, or `{`, found `;`
[01:02:39] 
[01:02:39] 
[01:02:39] The actual stderr differed from the expected stderr.
[01:02:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-variadic-function/invalid-variadic-function.stderr
[01:02:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-variadic-function/invalid-variadic-function.stderr
[01:02:39] To update references, rerun the tests and pass the `--bless` flag
[01:02:39] To only update this specific test, also pass `--test-args invalid/invalid-variadic-function.rs`
[01:02:39] error: 1 errors occurred comparing output.
[01:02:39] status: exit code: 1
[01:02:39] status: exit code: 1
[01:02:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-variadic-function.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-variadic-function/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-variadic-function/auxiliary" "-A" "unused"
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] stderr:
[01:02:39] stderr:
[01:02:39] ------------------------------------------
[01:02:39] {"message":"expected argument name, found `...`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/invalid/invalid-variadic-function.rs","byte_start":25,"byte_end":28,"line_start":1,"line_end":1,"column_start":26,"column_end":29,"is_primary":true,"text":[{"text":"extern \"C\" fn foo(x: u8, ...);","highlight_start":26,"highlight_end":29}],"label":"expected argument name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected argument name, found `...`\n  --> /checkout/src/test/ui/invalid/invalid-variadic-function.rs:1:26\n   |\nLL | extern \"C\" fn foo(x: u8, ...);\n   |                          ^^^ expected argument name\n\n"}
[01:02:39] {"message":"expected one of `->`, `where`, or `{`, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/invalid/invalid-variadic-function.rs","byte_start":29,"byte_end":30,"line_start":1,"line_end":1,"column_start":30,"column_end":31,"is_primary":true,"text":[{"text":"extern \"C\" fn foo(x: u8, ...);","highlight_start":30,"highlight_end":31}],"label":"expected one of `->`, `where`, or `{` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `->`, `where`, or `{`, found `;`\n  --> /checkout/src/test/ui/invalid/invalid-variadic-function.rs:1:30\n   |\nLL | extern \"C\" fn foo(x: u8, ...);\n   |                              ^ expected one of `->`, `where`, or `{` here\n\n"}
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] thread '[ui] ui/invalid/invalid-variadic-function.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] thread '[ui] ui/invalid/invalid-variadic-function.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] 
[01:02:39] ---- [ui] ui/parser/recover-enum2.rs stdout ----
[01:02:39] diff of stderr:
[01:02:39] 
[01:02:39] 10 LL |             Nope(i32 {}) //~ ERROR: found `{`
[01:02:39] 11    |                      ^ expected one of 7 possible tokens here
[01:02:39] 12 
[01:02:39] - error: expected one of `!`, `&&`, `&`, `(`, `)`, `*`, `+`, `,`, `::`, `<`, `?`, `[`, `_`, `crate`, `dyn`, `extern`, `fn`, `for`, `impl`, `pub`, `unsafe`, `}`, or lifetime, found `{`
[01:02:39] + error: expected one of `!`, `&&`, `&`, `(`, `)`, `*`, `+`, `,`, `...`, `::`, `<`, `?`, `[`, `_`, `crate`, `dyn`, `extern`, `fn`, `for`, `impl`, `pub`, `unsafe`, `}`, or lifetime, found `{`
[01:02:39] 15    |
[01:02:39] 15    |
[01:02:39] 16 LL |             Nope(i32 {}) //~ ERROR: found `{`
[01:02:39] -    |                      ^ expected one of 23 possible tokens here
[01:02:39] +    |                      ^ expected one of 24 possible tokens here
[01:02:39] 18 
[01:02:39] 19 error: expected expression, found reserved identifier `_`
[01:02:39] 19 error: expected expression, found reserved identifier `_`
[01:02:39] 20   --> $DIR/recover-enum2.rs:32:22
[01:02:39] 
[01:02:39] 
[01:02:39] The actual stderr differed from the expected stderr.
[01:02:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-enum2/recover-enum2.stderr
[01:02:39] To update references, rerun the tests and pass the `--bless` flag
[01:02:39] To only update this specific test, also pass `--test-args parser/recover-enum2.rs`
[01:02:39] error: 1 errors occurred comparing output.
[01:02:39] status: exit code: 1
[01:02:39] status: exit code: 1
[01:02:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-enum2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-enum2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-enum2/auxiliary" "-A" "unused"
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] stderr:
[01:02:39] stderr:
[01:02:39] ------------------------------------------
[01:02:39] {"message":"expected type, found `{`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-enum2.rs","byte_start":145,"byte_end":146,"line_start":8,"line_end":8,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"            abc: {}, //~ ERROR: expected type, found `{`","highlight_start":18,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected type, found `{`\n  --> /checkout/src/test/ui/parser/recover-enum2.rs:8:18\n   |\nLL |             abc: {}, //~ ERROR: expected type, found `{`\n   |                  ^\n\n"}
[01:02:39] {"message":"expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `{`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-enum2.rs","byte_start":421,"byte_end":422,"line_start":27,"line_end":27,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"            Nope(i32 {}) //~ ERROR: found `{`","highlight_start":22,"highlight_end":23}],"label":"expected one of 7 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `{`\n  --> /checkout/src/test/ui/parser/recover-enum2.rs:27:22\n   |\nLL |             Nope(i32 {}) //~ ERROR: found `{`\n   |                      ^ expected one of 7 possible tokens here\n\n"}
[01:02:39] {"message":"expected one of `!`, `&&`, `&`, `(`, `)`, `*`, `+`, `,`, `...`, `::`, `<`, `?`, `[`, `_`, `crate`, `dyn`, `extern`, `fn`, `for`, `impl`, `pub`, `unsafe`, `}`, or lifetime, found `{`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-enum2.rs","byte_start":421,"byte_end":422,"line_start":27,"line_end":27,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"            Nope(i32 {}) //~ ERROR: found `{`","highlight_start":22,"highlight_end":23}],"label":"expected one of 24 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `&&`, `&`, `(`, `)`, `*`, `+`, `,`, `...`, `::`, `<`, `?`, `[`, `_`, `crate`, `dyn`, `extern`, `fn`, `for`, `impl`, `pub`, `unsafe`, `}`, or lifetime, found `{`\n  --> /checkout/src/test/ui/parser/recover-enum2.rs:27:22\n   |\nLL |             Nope(i32 {}) //~ ERROR: found `{`\n   |                      ^ expected one of 24 possible tokens here\n\n"}
[01:02:39] {"message":"expected expression, found reserved identifier `_`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-enum2.rs","byte_start":557,"byte_end":558,"line_start":32,"line_end":32,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let bad_syntax = _; //~ ERROR: expected expression, found reserved identifier `_`","highlight_start":22,"highlight_end":23}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found reserved identifier `_`\n  --> /checkout/src/test/ui/parser/recover-enum2.rs:32:22\n   |\nLL |     let bad_syntax = _; //~ ERROR: expected expression, found reserved identifier `_`\n   |                      ^ expected expression\n\n"}
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] thread '[ui] ui/parser/recover-enum2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] thread '[ui] ui/parser/recover-enum2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] 
[01:02:39] ---- [ui] ui/parser/variadic-ffi-3.rs stdout ----
[01:02:39] diff of stderr:
[01:02:39] 
[01:02:39] - error: only foreign functions are allowed to be variadic
[01:02:39] + error: expected argument name, found `...`
[01:02:39] 3    |
[01:02:39] 3    |
[01:02:39] 4 LL | fn foo(x: isize, ...) {
[01:02:39] -    |                  ^^^
[01:02:39] +    |                  ^^^ expected argument name
[01:02:39] 6 
[01:02:39] 7 error: aborting due to previous error
[01:02:39] 7 error: aborting due to previous error
[01:02:39] 8 
[01:02:39] 
[01:02:39] 
[01:02:39] The actual stderr differed from the expected stderr.
[01:02:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-3/variadic-ffi-3.stderr
[01:02:39] To update references, rerun the tests and pass the `--bless` flag
[01:02:39] To only update this specific test, also pass `--test-args parser/variadic-ffi-3.rs`
[01:02:39] error: 1 errors occurred comparing output.
[01:02:39] status: exit code: 1
[01:02:39] status: exit code: 1
[01:02:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/variadic-ffi-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-3/auxiliary" "-A" "unused"
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] stderr:
[01:02:39] stderr:
[01:02:39] ------------------------------------------
[01:02:39] {"message":"expected argument name, found `...`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/variadic-ffi-3.rs","byte_start":17,"byte_end":20,"line_start":1,"line_end":1,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"fn foo(x: isize, ...) {","highlight_start":18,"highlight_end":21}],"label":"expected argument name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected argument name, found `...`\n  --> /checkout/src/test/ui/parser/variadic-ffi-3.rs:1:18\n   |\nLL | fn foo(x: isize, ...) {\n   |                  ^^^ expected argument name\n\n"}
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] thread '[ui] ui/parser/variadic-ffi-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] thread '[ui] ui/parser/variadic-ffi-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] 
[01:02:39] ---- [ui] ui/parser/variadic-ffi-4.rs stdout ----
[01:02:39] diff of stderr:
[01:02:39] 
[01:02:39] - error: only foreign functions are allowed to be variadic
[01:02:39] + error: expected argument name, found `...`
[01:02:39] 3    |
[01:02:39] 3    |
[01:02:39] 4 LL | extern "C" fn foo(x: isize, ...) {
[01:02:39] -    |                             ^^^
[01:02:39] +    |                             ^^^ expected argument name
[01:02:39] 6 
[01:02:39] 7 error: aborting due to previous error
[01:02:39] 7 error: aborting due to previous error
[01:02:39] 8 
[01:02:39] 
[01:02:39] 
[01:02:39] The actual stderr differed from the expected stderr.
[01:02:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-4/variadic-ffi-4.stderr
[01:02:39] To update references, rerun the tests and pass the `--bless` flag
[01:02:39] To only update this specific test, also pass `--test-args parser/variadic-ffi-4.rs`
[01:02:39] error: 1 errors occurred comparing output.
[01:02:39] status: exit code: 1
[01:02:39] status: exit code: 1
[01:02:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-4/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-4/auxiliary" "-A" "unused"
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] stderr:
[01:02:39] stderr:
[01:02:39] ------------------------------------------
[01:02:39] {"message":"expected argument name, found `...`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/variadic-ffi-4.rs","byte_start":28,"byte_end":31,"line_start":1,"line_end":1,"column_start":29,"column_end":32,"is_primary":true,"text":[{"text":"extern \"C\" fn foo(x: isize, ...) {","highlight_start":29,"highlight_end":32}],"label":"expected argument name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected argument name, found `...`\n  --> /checkout/src/test/ui/parser/variadic-ffi-4.rs:1:29\n   |\nLL | extern \"C\" fn foo(x: isize, ...) {\n   |                             ^^^ expected argument name\n\n"}
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] thread '[ui] ui/parser/variadic-ffi-4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] thread '[ui] ui/parser/variadic-ffi-4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:39] 
[01:02:39] ---- [ui] ui/variadic/variadic-ffi-3.rs stdout ----
[01:02:39] diff of stderr:
[01:02:39] 
[01:02:39] 23    |                                                        ^^^ expected non-variadic fn, found variadic function
[01:02:39] 24    |
[01:02:39] 25    = note: expected type `unsafe extern "C" fn(isize, u8)`
[01:02:39] -               found type `unsafe extern "C" fn(isize, u8, ...) {foo}`
[01:02:39] +               found type `for<'r> unsafe extern "C" fn(isize, u8, std::ffi::VaList<'r>, ...) {foo}`
[01:02:39] 28 error[E0308]: mismatched types
[01:02:39] 29   --> $DIR/variadic-ffi-3.rs:19:54
[01:02:39] 
[01:02:39] 
[01:02:39] 31 LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar;
[01:02:39] 32    |                                                      ^^^ expected variadic fn, found non-variadic function
[01:02:39] 33    |
[01:02:39] -    = note: expected type `extern "C" fn(isize, u8, ...)`
[01:02:39] +    = note: expected type `for<'r> extern "C" fn(isize, u8, std::ffi::VaList<'r>, ...)`
[01:02:39] 35               found type `extern "C" fn(isize, u8) {bar}`
[01:02:39] 36 
[01:02:39] 37 error[E0617]: can't pass `f32` to variadic function
[01:02:39] 
[01:02:39] The actual stderr differed from the expected stderr.
[01:02:39] The actual stderr differed from the expected stderr.
[01:02:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variadic/variadic-ffi-3/variadic-ffi-3.stderr
[01:02:39] To update references, rerun the tests and pass the `--bless` flag
[01:02:39] To only update this specific test, also pass `--test-args variadic/variadic-ffi-3.rs`
[01:02:39] error: 1 errors occurred comparing output.
[01:02:39] status: exit code: 1
[01:02:39] status: exit code: 1
[01:02:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variadic/variadic-ffi-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variadic/variadic-ffi-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variadic/variadic-ffi-3/auxiliary" "-A" "unused"
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] stderr:
[01:02:39] stderr:
[01:02:39] ------------------------------------------
[01:02:39] {"message":"this function takes at least 2 parameters but 0 parameters were supplied","code":{"code":"E0060","explanation":"\nExternal C functions are allowed to be variadic. However, a variadic function\ntakes a minimum number of arguments. For example, consider C's variadic `printf`\nfunction:\n\n