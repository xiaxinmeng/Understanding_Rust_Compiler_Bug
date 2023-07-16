\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs","byte_start":581,"byte_end":582,"line_start":18,"line_end":18,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    g(&mut b) //~ ERROR cannot borrow","highlight_start":12,"highlight_end":13}],"label":"cannot borrow mutably","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the `&mut`, as it is an immutable binding to a mutable reference","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs","byte_start":576,"byte_end":582,"line_start":18,"line_end":18,"column_start":7,"column_end":13,"is_primary":true,"text":[{"text":"    g(&mut b) //~ ERROR cannot borrow","highlight_start":7,"highlight_end":13}],"label":null,"suggested_replacement":"b","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable argument `b` as mutable\n  --> /checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs:18:12\n   |\nLL |     g(&mut b) //~ ERROR cannot borrow\n   |            ^ cannot borrow mutably\nhelp: consider removing the `&mut`, as it is an immutable binding to a mutable reference\n   |\nLL |     g(b) //~ ERROR cannot borrow\n   |       ^\n\n"}
[00:43:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:19] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] thread '[ui] ui/borrowck/mut-borrow-of-mut-ref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:43:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:19] 
[00:43:19] ---- [ui] ui/did_you_mean/issue-31424.rs stdout ----
[00:43:19] diff of stderr:
[00:43:19] 
[00:43:19] 10 error[E0596]: cannot borrow immutable argument `self` as mutable
[00:43:19] 11   --> $DIR/issue-31424.rs:23:15
[00:43:19] 12    |
[00:43:19] - LL |     fn bar(self: &mut Self) {
[00:43:19] -    |            --------------- consider changing this to `mut self: &mut Self`
[00:43:19] 15 LL |         (&mut self).bar(); //~ ERROR cannot borrow
[00:43:19] 16    |               ^^^^ cannot borrow mutably
[00:43:19] + help: consider removing the `&mut`, as it is an immutable binding to a mutable reference
[00:43:19] +    |
[00:43:19] + LL |         self: &mut Self.bar(); //~ ERROR cannot borrow
[00:43:19] 17 
[00:43:19] 18 error: aborting due to 2 previous errors
[00:43:19] 19 
[00:43:19] 
[00:43:19] 
[00:43:19] 
[00:43:19] The actual stderr differed from the expected stderr.
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-31424/issue-31424.stderr
[00:43:19] To update references, rerun the tests and pass the `--bless` flag
[00:43:19] To only update this specific test, also pass `--test-args did_you_mean/issue-31424.rs`
[00:43:19] 
[00:43:19] error: 1 errors occurred c reborrow mutably","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/did_you_mean/issue-31424.rs","byte_start":568,"byte_end":572,"line_start":17,"line_end":17,"column_start":15,"column_end":19,"is_primary":true,"text":[{"text":"        (&mut self).bar(); //~ ERROR cannot borrow","highlight_start":15,"highlight_end":19}],"label":"try removing `&mut` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0596]: cannot borrow immutable argument `self` as mutable\n  --> /checkout/src/test/ui/did_you_mean/issue-31424.rs:17:15\n   |\nLL |         (&mut self).bar(); //~ ERROR cannot borrow\n   |               ^^^^\n   |               |\n   |               cannot reborrow mutably\n   |               try removing `&mut` here\n\n"}
[00:43:19] {"message":"cannot borrow immutable argument `self` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n