\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":255,"byte_end":257,"line_start":11,"line_end":11,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        p = &x;","highlight_start":13,"highlight_end":15}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":263,"byte_end":264,"line_start":12,"line_end":12,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"`x` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":239,"byte_end":241,"line_start":10,"line_end":10,"column_start":21,"column_end":23,"is_primary":false,"text":[{"text":"        let x = 1 + *p;","highlight_start":21,"highlight_end":23}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `x` does not live long enough\n  --> /checkout/src/test/ui/span/regions-escape-loop-via-variable.rs:11:13\n   |\nLL |         let x = 1 + *p;\n   |                     -- borrow later used here\nLL |         p = &x;\n   |             ^^ borrowed value does not live long enough\nLL |     }\n   |     - `x` dropped here while still borrowed\n\n"}
[01:30:33] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] 
[01:30:33] thread '[ui (nll)] ui/span/regions-escape-loop-via-variable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] 
[01:30:33] ---- [ui (nll)] ui/span/regions-escape-loop-via-vec.rs stdout ----
[01:30:33] diff of stderr:
[01:30:33] 
[01:30:33] 26 LL |         _y.push(&mut z);
[01:30:33] 27    |         --      ^^^^^^ borrowed value does not live long enough
[01:30:33] 28    |         |
[01:30:33] -    |         borrow used here, in later iteration of loop
[01:30:33] 30 ...
[01:30:33] 31 LL |     }
[01:30:33] 31 LL |     }
[01:30:33] 32    |     - `z` dropped here while still borrowed
[01:30:33] 38    |                       ------ borrow of `x` occurs here
[01:30:33] 39 ...
[01:30:33] 39 ...
[01:30:33] 40 LL |         _y.push(&mut z);
[01:30:33] -    |         -- borrow used here, in later iteration of loop
[01:30:33] +    |         -- borrow later used here
[01:30:33] 42 LL |         //~^ ERROR `z` does not live long enough
[01:30:33] 43 LL |         x += 1; //~ ERROR cannot assign
[01:30:33] 44    |         ^^^^^^ use of borrowed `x`
[01:30:33] 
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/regions-escape-loop-via-vec.nll.stderr
[01:30:33] To update references, rerun the tests and pass the `--bless` flag
[01:30:33] To only update this specific test, also pass `--test-args span/regions-escape-loop-via-vec.rs`
[01:30:33] error: 1 errors occurred comparing output.
[01:30:33] status: exit code: 1
[01:30:33] status: exit code: 1
[01:30:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-escape-loop-via-vec.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/auxiliary" "-A" "unused"
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] stderr:
[01:30:33] stderr:
[01:30:33] ------------------------------------------
[01:30:33] {"message":"cannot use `x` because it was mutably borrowed","code":{"code":"E0503","explanation":"\nA value was used after it was mutably borrowed.\n\nExample of erroneous code:\n\n