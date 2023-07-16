\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":722,"byte_end":724,"line_start":21,"line_end":21,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        p = &x;","highlight_start":13,"highlight_end":15}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":730,"byte_end":731,"line_start":22,"line_end":22,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"`x` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":706,"byte_end":708,"line_start":20,"line_end":20,"column_start":21,"column_end":23,"is_primary":false,"text":[{"text":"        let x = 1 + *p;","highlight_start":21,"highlight_end":23}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `x` does not live long enough\n  --> /checkout/src/test/ui/span/regions-escape-loop-via-variable.rs:21:13\n   |\nLL |         let x = 1 + *p;\n   |                     -- borrow later used here\nLL |         p = &x;\n   |             ^^ borrowed value does not live long enough\nLL |     }\n   |     - `x` dropped here while still borrowed\n\n"}
[00:56:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:19] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] thread '[ui (nll)] ui/span/regions-escape-loop-via-variable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:56:19] 
[00:56:19] 
[00:56:19] ---- [ui (nll)] ui/span/regions-escape-loop-via-vec.rs stdout ----
[00:56:19] diff of stderr:
[00:56:19] 
[00:56:19] 7    |           ^ use of borrowed `x`
[00:56:19] 8 LL |         let mut z = x; //~ ERROR cannot use `x` because it was mutably borrowed
[00:56:19] 9 LL |         _y.push(&mut z);
[00:56:19] -    |         -- borrow used here, in later iteration of loop
[00:56:19] 11 
[00:56:19] 11 
[00:56:19] 12 error[E0503]: cannot use `x` because it was mutably borrowed
[00:56:19] 
[00:56:19] 
[00:56:19] 18 LL |         let mut z = x; //~ ERROR cannot use `x` because it was mutably borrowed
[00:56:19] 19    |                     ^ use of borrowed `x`
[00:56:19] 20 LL |         _y.push(&mut z);
[00:56:19] -    |         -- borrow used here, in later iteration of loop
[00:56:19] 22 
[00:56:19] 22 
[00:56:19] 23 error[E0597]: `z` does not live long enough
[00:56:19] 
[00:56:19] 
[00:56:19] 26 LL |         _y.push(&mut z);
[00:56:19] 27    |         --      ^^^^^^ borrowed value does not live long enough
[00:56:19] 28    |         |
[00:56:19] -    |         borrow used here, in later iteration of loop
[00:56:19] 30 ...
[00:56:19] 30 ...
[00:56:19] 31 LL |     }
[00:56:19] 32    |     - `z` dropped here while still borrowed
[00:56:19] 38    |                       ------ borrow of `x` occurs here
[00:56:19] 39 ...
[00:56:19] 39 ...
[00:56:19] 40 LL |         _y.push(&mut z);
[00:56:19] -    |         -- borrow used here, in later iteration of loop
[00:56:19] +    |         -- borrow later used here
[00:56:19] 42 LL |         //~^ ERROR `z` does not live long enough
[00:56:19] 43 LL |         x += 1; //~ ERROR cannot assign
[00:56:19] 44    |         ^^^^^^ use of borrowed `x`
[00:56:19] 
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/regions-escape-loop-via-vec.nll.stderr
[00:56:19] To update references, rerun the tests and pass the `--bless` flag
[00:56:19] To only update this specific test, also pass `--test-args span/regions-escape-loop-via-vec.rs`
[00:56:19] error: 1 errors occurred comparing output.
[00:56:19] status: exit code: 1
[00:56:19] status: exit code: 1
[00:56:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-escape-loop-via-vec.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/"highlight_end":11}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0503]: cannot use `x` because it was mutably borrowed\n  --> /checkout/src/test/ui/span/regions-escape-loop-via-vec.rs:15:11\n   |\nLL |     let mut _y = vec![&mut x];\n   |                       ------ borrow of `x` occurs here\nLL |     while x < 10 { //~ ERROR cannot use `x` because it was mutably borrowed\n   |           ^ use of borrowed `x`\nLL |         let mut z = x; //~ ERROR cannot use `x` because it was mutably borrowed\nLL |         _y.push(&mut z);\n   |         -- borrow later used here\n\n"}
[00:56:19] {"message":"cannot use `x` because it was mutably borrowed","code":{"code":"E0503","explanation":"\nA value was used after it was mutably borrowed.\n\nExample of erroneous code:\n\n