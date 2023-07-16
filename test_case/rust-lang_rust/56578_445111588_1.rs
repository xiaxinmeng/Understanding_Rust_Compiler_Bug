\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-const/associated-const-no-item.rs","byte_start":517,"byte_end":526,"line_start":16,"line_end":16,"column_start":16,"column_end":25,"is_primary":true,"text":[{"text":"const X: i32 = <i32>::ID;","highlight_start":BACKTRACE=1` for a backtrace.
[00:49:07] ---- [ui] ui/bogus-tag.rs stdout ----
[00:49:07] diff of stderr:
[00:49:07] 
[00:49:07] 
[00:49:07] 1 error[E0599]: no variant named `hsl` found for type `color` in the current scope
[00:49:07] -   --> $DIR/bogus-tag.rs:18:14
[00:49:07] +   --> $DIR/bogus-tag.rs:18:7
[00:49:07] 3    |
[00:49:07] 4 LL | enum color { rgb(isize, isize, isize), rgba(isize, isize, isize, isize), }
[00:49:07] 5    | ---------- variant `hsl` not found here
[00:49:07] 6 ...
[00:49:07] 6 ...
[00:49:07] 7 LL |       color::hsl(h, s, l) => { println!("hsl"); }
[00:49:07] -    |       -------^^^--------- variant not found in `color`
[00:49:07] +    |       ^^^^^^^^^^^^^^^^^^^ variant not found in `color`
[00:49:07] 10 error: aborting due to previous error
[00:49:07] 11 
[00:49:07] 
[00:49:07] 
[00:49:07] 
[00:49:07] The actual stderr differed from the expected stderr.
[00:49:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-tag/bogus-tag.stderr
[00:49:07] To update references, rerun the tests and pass the `--bless` flag
[00:49:07] To only update this specific test, also pass `--test-args bogus-tag.rs`
[00:49:07] error: 1 errors occurred comparing output.
[00:49:07] status: exit code: 1
[00:49:07] status: exit code: 1
[00:49:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bogus-tag.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-tag/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-99]: no variant named `hsl` found for type `color` in the current scope\n  --> /checkout/src/test/ui/bogus-tag.rs:18:7\n   |\nLL | enum color { rgb(isize, isize, isize), rgba(isize, isize, isize, isize), }\n   | ---------- variant `hsl` not found here\n...\nLL |       color::hsl(h, s, l) => { println!(\"hsl\"); }\n   |       ^^^^^^^^^^^^^^^^^^^ variant not found in `color`\n\n"}
[00:49:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:07] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[00:49:07] ------------------------------------------
[00:49:07] 
[00:49:07] thread '[ui] ui/bogus-tag.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:49:07] 
[00:49:07] 
[00:49:07] ---- [ui] ui/did_you_mean/bad-assoc-pat.rs stdout ----
[00:49:07] diff of stderr:
[00:49:07] 
[00:49:07] 23    |          ^^^^^^^^^^^^^^^^ help: try: `<(u8,)>::AssocItem`
[00:49:07] 24 
[00:49:07] 25 error[E0599]: no associated item named `AssocItem` found for type `[u8]` in the current scope
[00:49:07] +   --> $DIR/bad-assoc-pat.rs:13:9
[00:49:07] 27    |
[00:49:07] 27    |
[00:49:07] 28 LL |         [u8]::AssocItem => {}
[00:49:07] -    |         |
[00:49:07] -    |         associated item not found in `[u8]`
[00:49:07] +    |         ^^^^^^^^^^^^^^^ associated item not found in `[u8]`
[00:49:07] 32 
[00:49:07] 32 
[00:49:07] 33 error[E0599]: no associated item named `AssocItem` found for type `(u8, u8)` in the current scope
[00:49:07] +   --> $DIR/bad-assoc-pat.rs:16:9
[00:49:07] 35    |
[00:49:07] 35    |
[00:49:07] 36 LL |         (u8, u8)::AssocItem => {}
[00:49:07] -    |         |
[00:49:07] -    |         |
[00:49:07] -    |         associated item not found in `(u8, u8)`
[00:49:07] +    |         ^^^^^^^^^^^^^^^^^^^ associated item not found in `(u8, u8)`
[00:49:07] 40 
[00:49:07] 41 error[E0599]: no associated item named `AssocItem` found for type `_` in the current scope
[00:49:07] +   --> $DIR/bad-assoc-pat.rs:19:9
[00:49:07] 43    |
[00:49:07] 43    |
[00:49:07] 44 LL |         _::AssocItem => {}
[00:49:07] -    |         ---^^^^^^^^^
[00:49:07] -    |         associated item not found in `_`
[00:49:07] +    |         ^^^^^^^^^^^^ associated item not found in `_`
[00:49:07] 48 
[00:49:07] 48 
[00:49:07] 49 error[E0599]: no associated item named `AssocItem` found for type `(u8,)` in the current scope
[00:49:07] +   --> $DIR/bad-assoc-pat.rs:24:10
[00:49:07] 51    |
[00:49:07] 51    |
[00:49:07] 52 LL |         &(u8,)::AssocItem => {}
[00:49:07] -    |          |
[00:49:07] -    |          |
[00:49:07] -    |          associated item not found in `(u8,)`
[00:49:07] +    |          ^^^^^^^^^^^^^^^^ associated item not found in `(u8,)`
[00:49:07] 57 error: aborting due to 8 previous errors
[00:49:07] 58 
[00:49:07] 
[00:49:07] 
[0ans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":1112,"byte_end":1128,"line_start":24,"line_end":24,"column_start":10,"column_end":26,"is_primary":true,"text":[{"text":"        &(u8,)::AssocItem => {}","highlight_start":10,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":1112,"byte_end":1128,"line_start":24,"line_end":24,"column_start":10,"column_end":26,"is_primary":true,"text":[{"text":"        &(u8,)::AssocItem => {}","highlight_start":10,"highlight_end":26}],"label":null,"suggested_replacement":"<(u8,)>::AssocItem","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: missing angle brackets in associated item path\n  --> /checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs:24:10\n   |\nLL |         &(u8,)::AssocItem => {}\n   |          ^^^^^^^^^^^^^^^^ help: try: `<(u8,)>::AssocItem`\n\n"}
[00:49:07] {"message":"no associated item named `AssocItem` found for type `[u8]` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n