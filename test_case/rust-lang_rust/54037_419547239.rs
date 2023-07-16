plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:13] 
[00:45:13] running 2209 tests
[00:45:16] ........................................................................................F...........
[00:45:23] .....i..............................................................................................
[00:45:25] ....................................................................................................
[00:45:27] ....................................................................................................
[00:45:30] ....................................................................................................
---
[00:46:17] 
[00:46:17] ---- [ui] ui/cfg-attr-syntax-validation.rs stdout ----
[00:46:17] diff of stderr:
[00:46:17] 
[00:46:17] 1 error: `cfg` is not followed by parentheses
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:11:1
[00:46:17] 3    |
[00:46:17] 3    |
[00:46:17] 4 LL | #[cfg] //~ ERROR `cfg` is not followed by parentheses
[00:46:17] 5    | ^^^^^^ help: expected syntax is: `cfg(/* predicate */)`
[00:46:17] 6 
[00:46:17] 6 
[00:46:17] 7 error: `cfg` is not followed by parentheses
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:14:1
[00:46:17] 9    |
[00:46:17] 9    |
[00:46:17] 10 LL | #[cfg = 10] //~ ERROR `cfg` is not followed by parentheses
[00:46:17] 11    | ^^^^^^^^^^^ help: expected syntax is: `cfg(/* predicate */)`
[00:46:17] 12 
[00:46:17] 12 
[00:46:17] 13 error: `cfg` predicate is not specified
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:17:1
[00:46:17] 15    |
[00:46:17] 15    |
[00:46:17] 16 LL | #[cfg()] //~ ERROR `cfg` predicate is not specified
[00:46:17] 
[00:46:17] 18 
[00:46:17] 18 
[00:46:17] 19 error: multiple `cfg` predicates are specified
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:20:10
[00:46:17] 21    |
[00:46:17] 21    |
[00:46:17] 22 LL | #[cfg(a, b)] //~ ERROR multiple `cfg` predicates are specified
[00:46:17] 
[00:46:17] 24 
[00:46:17] 24 
[00:46:17] 25 error: `cfg` predicate key cannot be a literal
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:23:7
[00:46:17] 27    |
[00:46:17] 27    |
[00:46:17] 28 LL | #[cfg("str")] //~ ERROR `cfg` predicate key cannot be a literal
[00:46:17] 
[00:46:17] 30 
[00:46:17] 30 
[00:46:17] 31 error: `cfg` predicate key must be an identifier
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:26:7
[00:46:17] 33    |
[00:46:17] 33    |
[00:46:17] 34 LL | #[cfg(a::b)] //~ ERROR `cfg` predicate key must be an identifier
[00:46:17] 
[00:46:17] 36 
[00:46:17] 36 
[00:46:17] 37 error[E0537]: invalid predicate `a`
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:29:7
[00:46:17] 39    |
[00:46:17] 39    |
[00:46:17] 40 LL | #[cfg(a())] //~ ERROR invalid predicate `a`
[00:46:17] 
[00:46:17] 42 
[00:46:17] 42 
[00:46:17] 43 error: literal in `cfg` predicate value must be a string
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:32:11
[00:46:17] 45    |
[00:46:17] 45    |
[00:46:17] 46 LL | #[cfg(a = 10)] //~ ERROR literal in `cfg` predicate value must be a string
[00:46:17] 
[00:46:17] 48 
[00:46:17] 48 
[00:46:17] 49 error: `cfg` is not a well-formed meta-item
[00:46:17] +   --> $DIR/cfg-attr-syntax-validation.rs:37:9
[00:46:17] 51    |
[00:46:17] 51    |
[00:46:17] 52 LL |         #[cfg(feature = $expr)] //~ ERROR `cfg` is not a well-formed meta-item
[00:46:17] 53    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: expected syntax is: `#[cfg(/* predicate */)]`
[00:46:17] 
[00:46:17] The actual stderr differed from the expected stderr.
[00:46:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg-attr-syntax-validation/cfg-attr-syntax-validation.stderr
[00:46:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg-attr-syntax-validation/cfg-attr-syntax-validation.stderr
[00:46:17] To update references, rerun the tests and pass the `--bless` flag
[00:46:17] To only update this specific test, also pass `--test-args cfg-attr-syntax-validation.rs`
[00:46:17] error: 1 errors occurred comparing output.
[00:46:17] status: exit code: 1
[00:46:17] status: exit code: 1
[00:46:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg-attr-syntax-validation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg-attr-syntax-validation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg-attr-syntax-validation/auxiliary" "-A" "unused"
[00:46:17] ------------------------------------------
[00:46:17] 
[00:46:17] ------------------------------------------
[00:46:17] stderr:
[00:46:17] stderr:
[00:46:17] ------------------------------------------
[00:46:17] {"message":"`cfg` is not followed by parentheses","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":467,"byte_end":473,"line_start":11,"line_end":11,"column_start":1,"column_end":7,"is_primary":true,"text":[{"text":"#[cfg] //~ ERROR `cfg` is not followed by parentheses","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected syntax is","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":467,"byte_end":473,"line_start":11,"line_end":11,"column_start":1,"column_end":7,"is_primary":true,"text":[{"text":"#[cfg] //~ ERROR `cfg` is not followed by parentheses","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"cfg(/* predicate */)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `cfg` is not followed by parentheses\n  --> /checkout/src/test/ui/cfg-attr-syntax-validation.rs:11:1\n   |\nLL | #[cfg] //~ ERROR `cfg` is not followed by parentheses\n   | ^^^^^^ help: expected syntax is: `cfg(/* predicate */)`\n\n"}
[00:46:17] {"message":"`cfg` is not followed by parentheses","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":533,"byte_end":544,"line_start":14,"line_end":14,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"#[cfg = 10] //~ ERROR `cfg` is not followed by parentheses","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected syntax is","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":533,"byte_end":544,"line_start":14,"line_end":14,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"#[cfg = 10] //~ ERROR `cfg` is not followed by parentheses","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":"cfg(/* predicate */)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `cfg` is not followed by parentheses\n  --> /checkout/src/test/ui/cfg-attr-syntax-validation.rs:14:1\n   |\nLL | #[cfg = 10] //~ ERROR `cfg` is not followed by parentheses\n   | ^^^^^^^^^^^ help: expected syntax is: `cfg(/* predicate */)`\n\n"}
[00:46:17] {"message":"`cfg` predicate is not specified","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":604,"byte_end":612,"line_start":17,"line_end":17,"column_start":1,"column_end":9,"is_primary":true,"text":[{"text":"#[cfg()] //~ ERROR `cfg` predicate is not specified","highlight_start":1,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `cfg` predicate is not specified\n  --> /checkout/src/test/ui/cfg-attr-syntax-validation.rs:17:1\n   |\nLL | #[cfg()] //~ ERROR `cfg` predicate is not specified\n   | ^^^^^^^^\n\n"}
[00:46:17] {"message":"multiple `cfg` predicates are specified","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":677,"byte_end":678,"line_start":20,"line_end":20,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"#[cfg(a, b)] //~ ERROR multiple `cfg` predicates are specified","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: multiple `cfg` predicates are specified\n  --> /checkout/src/test/ui/cfg-attr-syntax-validation.rs:20:10\n   |\nLL | #[cfg(a, b)] //~ ERROR multiple `cfg` predicates are specified\n   |          ^\n\n"}
[00:46:17] {"message":"`cfg` predicate key cannot be a literal","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":749,"byte_end":754,"line_start":23,"line_end":23,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"#[cfg(\"str\")] //~ ERROR `cfg` predicate key cannot be a literal","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `cfg` predicate key cannot be a literal\n  --> /checkout/src/test/ui/cfg-attr-syntax-validation.rs:23:7\n   |\nLL | #[cfg(\"str\")] //~ ERROR `cfg` predicate key cannot be a literal\n   |       ^^^^^\n\n"}
[00:46:17] {"message":"`cfg` predicate key must be an identifier","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cfg-attr-syntax-validation.rs","byte_start":825,"byte_end":829,"line_start":26,"line_end":26,"column_start":7,"column_end":11,"is_primary":true,"text":[{"text":"#[cfg(a::b)] //~ ERROR `cfg` predicate key must be an identifier","highlight_start":7,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `cfg` predicate key must be an identifier\n  --> /checkout/src/test/ui/cfg-attr-syntax-validation.rs:26:7\n   |\nLL | #[cfg(a::b)] //~ ERROR `cfg` predicate key must be an identifier\n   |       ^^^^\n\n"}
[00:46:17] {"message":"invalid predicate `a`","code":{"code":"E0537","explanation":"\nAn unknown predicate was used inside the `cfg` attribute.\n\nErroneous code example:\n\n