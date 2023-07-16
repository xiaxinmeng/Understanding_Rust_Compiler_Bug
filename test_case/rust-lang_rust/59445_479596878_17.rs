\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":54,"byte_end":68,"line_start":3,"line_end":3,"column_start":29,"column_end":43,"is_primary":false,"text":[{"text":"trait Foo = std::io::Read + std::io::Write;","highlight_start":29,"highlight_end":43}],"label":"additional non-auto trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":38,"byte_end":51,"line_start":3,"line_end":3,"column_start":13,"column_end":26,"is_primary":false,"text":[{"text":"trait Foo = std::io::Read + std::io::Write;","highlight_start":13,"highlight_end":26}],"label":"first non-auto trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":237,"byte_end":240,"line_start":8,"line_end":8,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    let _: Box<Foo>;","highlight_start":16,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0225]: only auto traits can be used as additional traits in a trait object\n  --> /checkout/src/test/ui/error-codes/E0225.rs:8:16\n   |\nLL | trait Foo = std::io::Read + std::io::Write;\n   |             -------------   -------------- additional non-auto trait\n   |             |\n   |             first non-auto trait\n...\nLL |     let _: Box<Foo>;\n   |                ^^^\n\n"}
[01:15:35] {"message":"For more information about this error, try `rustc --explain E0225`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0225`.\n"}
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] 
[01:15:35] thread '[ui] ui/error-codes/E0225.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] 
[01:15:35] ---- [ui] ui/issues/issue-22560.rs stdout ----
[01:15:35] diff of stderr:
[01:15:35] 
[01:15:35] 1 error[E0393]: the type parameter `Rhs` must be explicitly specified
[01:15:35] +   --> $DIR/issue-22560.rs:8:13
[01:15:35] 3    |
[01:15:35] 3    |
[01:15:35] - LL | type Test = Add +
[01:15:35] + LL |             Sub;
[01:15:35] 5    |             ^^^ missing reference to `Rhs`
[01:15:35] 6    |
[01:15:35] 7    = note: because of the default `Self` reference, type parameters must be specified on object types
[01:15:35] 8 
[01:15:35] 8 
[01:15:35] 9 error[E0393]: the type parameter `Rhs` must be explicitly specified
[01:15:35] +   --> $DIR/issue-22560.rs:5:13
[01:15:35] 11    |
[01:15:35] - LL |             Sub;
[01:15:35] + LL | type Test = Add +
[01:15:35] + LL | type Test = Add +
[01:15:35] 13    |             ^^^ missing reference to `Rhs`
[01:15:35] 14    |
[01:15:35] 15    = note: because of the default `Self` reference, type parameters must be specified on object types
[01:15:35] 
[01:15:35] 17 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:15:35] 19    |
[01:15:35] + LL | type Test = Add +
[01:15:35] +    |             --- first non-auto trait
[01:15:35] + ...
[01:15:35] + ...
[01:15:35] 20 LL |             Sub;
[01:15:35] -    |             ^^^ non-auto additional trait
[01:15:35] +    |             ^^^ additional non-auto trait
[01:15:35] 22 
[01:15:35] - error[E0191]: the value of the associated type `Output` (from the trait `std::ops::Add`) must be specified
[01:15:35] + error[E0191]: the value of the associated types `Output` (from the trait `std::ops::Add`), `Output` (from the trait `std::ops::Sub`) must be specified
[01:15:35] 25    |
[01:15:35] 26 LL |   type Test = Add +
[01:15:35] 
[01:15:35] 27    |  _____________^
[01:15:35] 27    |  _____________^
[01:15:35] +    | |_____________|
[01:15:35] +    | |
[01:15:35] 28 LL | |
[01:15:35] 29 LL | |
[01:15:35] 30 LL | |             Sub;
[01:15:35] 
[01:15:35] -    | |_______________^ associated type `Output` must be specified
[01:15:35] +    | |_______________|
[01:15:35] +    | |_______________|
[01:15:35] +    | |_______________associated type `Output` must be specified
[01:15:35] +    |                 associated type `Output` must be specified
[01:15:35] 33 error: aborting due to 4 previous errors
[01:15:35] 34 
[01:15:35] 
[01:15:35] 
[01:15:35] 
[01:15:35] The actual stderr differed from the expected stderr.
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/issue-22560.stderr
[01:15:35] To update references, rerun the tests and pass the `--bless` flag
[01:15:35] To only update this specific test, also pass `--test-args issues/issue-22560.rs`
[01:15:35] error: 1 errors occurred comparing output.
[01:15:35] status: exit code: 1
[01:15:35] status: exit code: 1
[01:15:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22560.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/auxiliary" "-A" "unused"
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] stderr:
[01:15:35] stderr:
[01:15:35] ------------------------------------------
[01:15:35] {"message":"the type parameter `Rhs` must be explicitly specified","code":{"code":"E0393","explanation":"\nA type parameter which references `Self` in its default value was not specified.\nExample of erroneous code:\n\n