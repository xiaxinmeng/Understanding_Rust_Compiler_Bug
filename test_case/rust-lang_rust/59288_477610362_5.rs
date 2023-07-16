\n\nAn `if` expression without an `else` block has the type `()`, so this is a type\nerror. To resolve it, add an `else` block having the same type as the `if`\nblock.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-4201.rs","byte_start":55,"byte_end":209,"line_start":4,"line_end":10,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else if false {","highlight_start":12,"highlight_end":22},{"text":"//~^ ERROR if may be missing an else clause","highlight_start":1,"highlight_end":44},{"text":"//~| expected type `()`","highlight_start":1,"highlight_end":24},{"text":"//~| found type `{integer}`","highlight_start":1,"highlight_end":28},{"text":"//~| expected (), found integer","highlight_start":1,"highlight_end":32},{"text":"        1","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"expected (), found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0317]: if may be missing an else clause\n  --> /checkout/src/test/ui/issues/issue-4201.rs:4:12\n   |\nLL |       } else if false {\n   |  ____________^\nLL | | //~^ ERROR if may be missing an else clause\nLL | | //~| expected type `()`\nLL | | //~| found type `{integer}`\nLL | | //~| expected (), found integer\nLL | |         1\nLL | |     };\n   | |_____^ expected (), found integer\n   |\n   = note: expected type `()`\n              found type `{integer}`\n\n"}
[01:14:46] {"message":"For more information about this error, try `rustc --explain E0317`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0317`.\n"}
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] 
[01:14:46] 
[01:14:46] thread '[ui] ui/issues/issue-4201.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:14:46] 
[01:14:46] ---- [ui] ui/issues/issue-50577.rs stdout ----
[01:14:46] diff of stderr:
[01:14:46] 
[01:14:46] 2   --> $DIR/issue-50577.rs:3:16
[01:14:46] 3    |
[01:14:46] 4 LL |         Drop = assert_eq!(1, 1)
[01:14:46] -    |                |
[01:14:46] -    |                expected (), found isize
[01:14:46] -    |                found here
[01:14:46] +    |                ^^^^^^^^^^^^^^^^ expected (), found isize
[01:14:46] +    |                ^^^^^^^^^^^^^^^^ expected (), found isize
[01:14:46] 9    |
[01:14:46] 10    = note: expected type `()`
[01:14:46] 11               found type `isize`
[01:14:46] 
[01:14:46] -    = note: `if` expressions without `else` evaluate to `()`
[01:14:46] 14    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:14:46] 15 
[01:14:46] 16 error: aborting due to previous error
[01:14:46] 
[01:14:46] 
[01:14:46] 
[01:14:46] The actual stderr differed from the expected stderr.
[01:14:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/issue-50577.stderr
[01:14:46] To update references, rerun the tests and pass the `--bless` flag
[01:14:46] To only update this specific test, also pass `--test-args issues/issue-50577.rs`
[01:14:46] error: 1 errors occurred comparing output.
[01:14:46] status: exit code: 1
[01:14:46] status: exit code: 1
[01:14:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50577.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/auxiliary" "-A" "unused"
[01:14:46] ------------------------------------------
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] stderr:
[01:14:46] stderr:
[01:14:46] ------------------------------------------
[01:14:46] {"message":"if may be missing an else clause","code":{"code":"E0317","explanation":"\nThis error occurs when an `if` expression without an `else` block is used in a\ncontext where a type other than `()` is expected, for example a `let`\nexpression:\n\n