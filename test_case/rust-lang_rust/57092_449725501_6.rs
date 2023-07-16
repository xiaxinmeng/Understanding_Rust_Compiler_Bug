\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/access-mode-in-closures.rs","byte_start":647,"byte_end":649,"line_start":18,"line_end":18,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"        match *s { S(v) => v } //~ ERROR cannot move out","highlight_start":15,"highlight_end":17}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/access-mode-in-closures.rs","byte_start":654,"byte_end":655,"line_start":18,"line_end":18,"column_start":22,"column_end":23,"is_primary":false,"text":[{"text":"        match *s { S(v) => v } //~ ERROR cannot move out","highlight_start":22,"highlight_end":23}],"label":"data moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `v` has type `std::vec::Vec<isize>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/access-mode-in-closures.rs","byte_start":654,"byte_end":655,"line_start":18,"line_end":18,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"        match *s { S(v) => v } //~ ERROR cannot move out","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider removing the `*`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/access-mode-in-closures.rs","byte_start":647,"byte_end":649,"line_start":18,"line_end":18,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"        match *s { S(v) => v } //~ ERROR cannot move out","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":"s","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/access-mode-in-closures.rs:18:15\n   |\nLL |         match *s { S(v) => v } //~ ERROR cannot move out\n   |               ^^     - data moved here\n   |               |\n   |               cannot move out of borrowed content\n   |               help: consider removing the `*`: `s`\n   |\nnote: move occurs because `v` has type `std::vec::Vec<isize>`, which does not implement the `Copy` trait\n  --> /checkout/src/test/ui/access-mode-in-closures.rs:18:22\n   |\nLL |         match *s { S(v) => v } //~ ERROR cannot move out\n   |                      ^\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/access-mode-in-closures.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:20:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/borrowck/borrowck-assign-comp.rs#ast stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 15    |
[01:20:27] 16 LL |     let q = &p.y;
[01:20:27] 17    |             ---- borrow of `p` occurs here
[01:20:27] - LL |     p = point {x: 5, y: 7};//[ast]~ ERROR cannot assign to `p`
[01:20:27] + LL |     p = Point {x: 5, y: 7};//[ast]~ ERROR cannot assign to `p`
[01:20:27] 19    |     ^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `p` occurs here
[01:20:27] 20 ...
[01:20:27] 21 LL |     *q; // stretch loan
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-comp.ast.nll/borrowck-assign-comp.ast.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args borrowck/borrowck-assign-comp.rs`
[01:20:27] 
[01:20:27] error in revision `ast`: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-assign-comp.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-comp.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-comp.ast.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"cannot assign to `p.x` because it is borrowed","code":{"code":"E0506","explanation":"\nThis error occurs when an attempt is made to assign to a borrowed value.\n\nExample of erroneous code:\n\n