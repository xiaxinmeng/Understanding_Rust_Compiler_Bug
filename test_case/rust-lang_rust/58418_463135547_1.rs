\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-14092.rs","byte_start":10,"byte_end":13,"line_start":1,"line_end":1,"column_start":11,"column_end":14,"is_primary":true,"text":[{"text":"fn fn1(0: Box) {}","highlight_start":11,"highlight_end":14}],"label":"expected at least 1 type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected at least 1, found 0\n  --> /checkout/src/test/ui/issues/issue-14092.rs:1:11\n   |\nLL | fn fn1(0: Box) {}\n   |           ^^^ expected at least 1 type argument\n\n"}
[01:01:00] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[01:01:00] 
[01:01:00] ------------------------------------------
[01:01:00] 
[01:01:00] 
[01:01:00] thread '[ui] ui/issues/issue-14092.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:01:00] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:01:00] 
[01:01:00] ---- [ui] ui/issues/issue-41974.rs stdout ----
[01:01:00] diff of stderr:
[01:01:00] 
[01:01:00] - error[E0119]: conflicting implementations of trait `std::ops::Drop` for type `std::boxed::Box<_>`:
[01:01:00] + error[E0119]: conflicting implementations of trait `std::ops::Drop` for type `std::boxed::Box<_, _>`:
[01:01:00] 3    |
[01:01:00] 3    |
[01:01:00] 4 LL | impl<T> Drop for T where T: A { //~ ERROR E0119
[01:01:00] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:00] 6    |
[01:01:00] 7    = note: conflicting implementation in crate `alloc`:
[01:01:00] 7    = note: conflicting implementation in crate `alloc`:
[01:01:00] -            - impl<T> std::ops::Drop for std::boxed::Box<T>
[01:01:00] -              where T: ?Sized;
[01:01:00] -    = note: downstream crates may implement trait `A` for type `std::boxed::Box<_>`
[01:01:00] +            - impl<T, A> std::ops::Drop for std::boxed::Box<T, A>
[01:01:00] +              where A: std::default::Default, A: std::alloc::Alloc, T: ?Sized;
[01:01:00] +    = note: downstream crates may implement trait `A` for type `std::boxed::Box<_, _>`
[01:01:00] 11 
[01:01:00] 12 error[E0120]: the Drop trait may only be implemented on structures
[01:01:00] 
[01:01:00] 
[01:01:00] The actual stderr differed from the expected stderr.
[01:01:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/issue-41974.stderr
[01:01:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/issue-41974.stderr
[01:01:00] To update references, rerun the tests and pass the `--bless` flag
[01:01:00] To only update this specific test, also pass `--test-args issues/issue-41974.rs`
[01:01:00] error: 1 errors occurred comparing output.
[01:01:00] status: exit code: 1
[01:01:00] status: exit code: 1
[01:01:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41974.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/auxiliary" "-A" "unused"
[01:01:00] ------------------------------------------
[01:01:00] 
[01:01:00] ------------------------------------------
[01:01:00] stderr:
[01:01:00] stderr:
[01:01:00] ------------------------------------------
[01:01:00] {"message":"conflicting implementations of trait `std::ops::Drop` for type `std::boxed::Box<_, _>`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n