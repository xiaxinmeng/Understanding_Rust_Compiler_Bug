\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/borrowing.rs","byte_start":776,"byte_end":865,"line_start":24,"line_end":27,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        || {","highlight_start":9,"highlight_end":13},{"text":"            yield &a","highlight_start":1,"highlight_end":21},{"text":"            //~^ ERROR: `a` does not live long enough","highlight_start":1,"highlight_end":54},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":"borrowed value does not live long enough","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/borrowing.rs","byte_start":870,"byte_end":871,"line_start":28,"line_end":28,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    };","highlight_start":5,"highlight_end":6}],"label":"borrowed value only lives until here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/borrowing.rs","byte_start":873,"byte_end":874,"line_start":29,"line_end":29,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"borrow later used here, when `_b` is dropped","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `a` does not live long enough\n  --> /checkout/src/test/ui/generator/borrowing.rs:24:9\n   |\nLL | /         || {\nLL | |             yield &a\nLL | |             //~^ ERROR: `a` does not live long enough\nLL | |         }\n   | |_________^ borrowed value does not live long enough\nLL |       };\n   |       - borrowed value only lives until here\nLL |   }\n   |   - borrow later used here, when `_b` is dropped\n\n"}
[00:47:47] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:47] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui (nll)] ui/generator/borrowing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:47:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:47] 
[00:47:47] ---- [ui (nll)] ui/generator/dropck.rs stdout ----
[00:47:47]  diff of stderr:
[00:47:47] 
[00:47:47] - error: compilation successful
[00:47:47] -   --> $DIR/dropck.rs:16:1
[00:47:47] + error[E0597]: `ref_` does not live long enough
[00:47:47] +   --> $DIR/dropck.rs:22:11
[00:47:47] 3    |
[00:47:47] - LL | / fn main() { #![rustc_error] // rust-lang/rust#49855
[00:47:47] - LL | |     let (cell, mut gen);
[00:47:47] - LL | |     cell = Box::new(RefCell::new(0));
[00:47:47] - LL | |     let ref_ = Box::leak(Box::new(Some(cell.borrow_mut())));
[00:47:47] - ...  |
[00:47:47] - LL | |     // drops the RefCell and then the Ref, leading to use-after-free
[00:47:47] - LL | | }
[00:47:47] -    | |_^
[00:47:47] + LL |       gen = || {
[00:47:47] +    |  ___________^
[00:47:47] + LL | |         // but the generator can use it to drop a `Ref<'a, i32>`.
[00:47:47] + LL | |         let _d = ref_.take(); //~ ERROR `ref_` does not livestc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui (nll)] ui/generator/dropck.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:47:47] 
[00:47:47] 
[00:47:47] ---- [ui (nll)] ui/generator/ref-escapes-but-not-over-yield.rs stdout ----
[00:47:47]  diff of stderr:
[00:47:47] 
[00:47:47] 1 error[E0597]: `b` does not live long enough
[00:47:47] 3    |
[00:47:47] 3    |
[00:47:47] - LL |         a = &b;
[00:47:47] -    |             ^^ borrowed value does not live long enough
[00:47:47] - LL |         //~^ ERROR `b` does not live long enough
[00:47:47] - LL |     };
[00:47:47] -    |     - borrowed value only lives until here
[00:47:47] + LL |       let mut b = move || {
[00:47:47] +    |  _________________-
[00:47:47] + LL | |         yield();
[00:47:47] + LL | |         let b = 5;
[00:47:47] + LL | |         a = &b;
[00:47:47] +    | |             ^^ borrowed value does not live long enough
[00:47:47] + LL | |         //~^ ERROR `b` does not live long enough
[00:47:47] + LL | |     };
[00:47:47] +    | |     |
[00:47:47] +    | |     |
[00:47:47] +    | |     borrowed value only lives until here
[00:47:47] +    | |_____temporary later dropped here, potentially using the reference
[00:47:47] +    |       borrow may end up in a temporary, created here
[00:47:47] 10 error: aborting due to previous error
[00:47:47] 11 
[00:47:47] 
[00:47:47] 
[00:47:47] 
[00:47:47] The actual stderr differed from the expected stderr.
[00:47:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/ref-escapes-but-not-over-yield.nll.stderr
[00:47:47] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'generator/ref-escapes-but-not-over-yield.rs'
[00:47:47] 
[00:47:47] error: 1 errors occurred comparing output.
[00:47:47] status: exit code: 101
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/ref-escapes-but-not-over-yield.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/ref-escapes-but-not-over-yield.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/ref-escapes-but-not-over-yield.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"`b` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n