\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs","byte_start":711,"byte_end":720,"line_start":20,"line_end":20,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"    let (ref mut x, ) = (1234543, ); //~ ERROR","highlight_start":10,"highlight_end":19}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs","byte_start":755,"byte_end":756,"line_start":22,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"borrowed value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs:20:10\n   |\nLL |     let (ref mut x, ) = (1234543, ); //~ ERROR\n   |          ^^^^^^^^^ borrowed value does not live long enough\nLL |     x\nLL | }\n   | - borrowed value only lives until here\replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs:37:10\n   |\nLL |     &mut 1234543 //~ ERROR\n   |          ^^^^^^^ temporary value does not live long enough\nLL | }\n   | - temporary value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:40:29] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:40:29] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:40:29] ------------------------------------------
[00:40:29] 
[00:40:29] thread '[ui] ui/borrowck/promote-ref-mut-in-let-issue-46557.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:40:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:40:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:40:29] 
[00:40:29] ---- [ui] ui/const-eval/promoted_const_fn_fail.rs stdout ----
[00:40:29] diff of stderr:
[00:40:29] 
[00:40:29] - error: this expression will panic at runtime
[00:40:29] -   --> $DIR/promoted_const_fn_fail.rs:25:9
[00:40:29] + error[E0597]: borrowed value does not live long enough
[00:40:29] +   --> $DIR/promoted_const_fn_fail.rs:35:27
[00:40:29] 3    |
[00:40:29] - LL |         Bar { a: &42 }.b as u8
[00:40:29] -    |         ^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
[00:40:29] + LL |     let x: &'static u8 = &(bar() + 1);
[00:40:29] +    |                           ^^^^^^^^^^^ temporary value does not live long enough
[00:40:29] + ...
[00:40:29] + LL | }
[00:40:29] +    | - temporary value only lives until here
[00:40:29] - note: lint level defined here
[00:40:29] -   --> $DIR/promoted_const_fn_fail.rs:13:9
[00:40:29] -    |
[00:40:29] -    |
[00:40:29] - LL | #![deny(const_err)]
[00:40:29] -    |         ^^^^^^^^^
[00:40:29] +    = note: borrowed value must be valid for the static lifetime...
[00:40:29] 12 
[00:40:29] - error: this expression will panic at runtime
[00:40:29] -    |
[00:40:29] -    |
[00:40:29] - LL |         Bar { a: &42 }.b as u8
[00:40:29] -    |         ^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
[00:40:29] + error: aborting due to previous error
[00:40:29] - error: aborting due to 2 previous errors
[00:40:29] - 
[00:40:29] + For more information about this error, try `rustc --explain E0597`.
[00:40:29] 21 
[00:40:29] 21 
[00:40:29] 
[00:40:29] 
[00:40:29] The actual stderr differed from the expected stderr.
[00:40:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/promoted_const_fn_fail/promoted_const_fn_fail.stderr
[00:40:29] To update references, rerun the tests and pass the `--bless` flag
[00:40:29] To only up\nthe `x`'s one:\n\n