plain
2019-08-31T12:04:24.1957966Z test [ui] ui/substs-ppaux.rs#verbose ... ok
2019-08-31T12:04:24.2069637Z test [ui] ui/suffixed-literal-meta.rs ... ok
2019-08-31T12:04:24.2668481Z test [ui] ui/suggestions/assoc-const-as-field.rs ... ok
2019-08-31T12:04:24.3320836Z test [ui] ui/suggestions/assoc-type-in-method-return.rs ... ok
2019-08-31T12:04:24.4005383Z test [ui] ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs ... ok
2019-08-31T12:04:24.4625978Z test [ui] ui/suggestions/attribute-typos.rs ... ok
2019-08-31T12:04:24.5501624Z test [ui] ui/suggestions/borrow-for-loop-head.rs ... ok
2019-08-31T12:04:24.6336070Z test [ui] ui/suggestions/dont-suggest-deref-inside-macro-issue-58298.rs ... ok
2019-08-31T12:04:24.7026173Z test [ui] ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs ... ok
2019-08-31T12:04:24.7026173Z test [ui] ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs ... ok
2019-08-31T12:04:24.8129363Z test [ui] ui/suggestions/dont-suggest-ref/move-into-closure.rs ... ok
2019-08-31T12:04:24.9225712Z test [ui] ui/suggestions/dont-suggest-ref/simple.rs ... ok
2019-08-31T12:04:24.9809946Z test [ui] ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs ... ok
2019-08-31T12:04:25.0430186Z test [ui] ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs ... ok
2019-08-31T12:04:25.1178402Z test [ui] ui/suggestions/format-borrow.rs ... ok
2019-08-31T12:04:25.1702406Z test [ui] ui/suggestions/impl-trait-return-trailing-semicolon.rs ... ok
2019-08-31T12:04:25.1882614Z test [ui] ui/suggestions/fn-or-tuple-struct-without-args.rs ... ok
---
2019-08-31T12:16:46.9007679Z test [ui (nll)] ui/substs-ppaux.rs#verbose ... ok
2019-08-31T12:16:46.9008348Z test [ui (nll)] ui/suffixed-literal-meta.rs ... ok
2019-08-31T12:16:46.9008823Z test [ui (nll)] ui/suggestions/assoc-const-as-field.rs ... ok
2019-08-31T12:16:46.9009300Z test [ui (nll)] ui/suggestions/assoc-type-in-method-return.rs ... ok
2019-08-31T12:16:46.9010398Z test [ui (nll)] ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs ... ok
2019-08-31T12:16:46.9011870Z test [ui (nll)] ui/suggestions/attribute-typos.rs ... ok
2019-08-31T12:16:46.9012400Z test [ui (nll)] ui/suggestions/borrow-for-loop-head.rs ... ok
2019-08-31T12:16:46.9012973Z test [ui (nll)] ui/suggestions/dont-suggest-deref-inside-macro-issue-58298.rs ... ok
2019-08-31T12:16:46.9013540Z test [ui (nll)] ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs ... ok
2019-08-31T12:16:46.9013540Z test [ui (nll)] ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs ... ok
2019-08-31T12:16:46.9330503Z test [ui (nll)] ui/suggestions/dont-suggest-ref/move-into-closure.rs ... ok
2019-08-31T12:16:47.0665493Z test [ui (nll)] ui/suggestions/dont-suggest-ref/simple.rs ... ok
2019-08-31T12:16:47.0984583Z test [ui (nll)] ui/suggestions/dont-suggest-try_into-in-macros.rs ... ok
2019-08-31T12:16:47.1221179Z test [ui (nll)] ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs ... ok
2019-08-31T12:16:47.2191739Z test [ui (nll)] ui/suggestions/format-borrow.rs ... ok
2019-08-31T12:16:47.2669251Z test [ui (nll)] ui/suggestions/impl-trait-return-trailing-semicolon.rs ... ok
2019-08-31T12:16:47.3100460Z test [ui (nll)] ui/suggestions/fn-or-tuple-struct-without-args.rs ... ok
2019-08-31T12:16:47.3678269Z test [ui (nll)] ui/suggestions/impl-trait-with-missing-trait-bounds-in-arg.rs ... ok
---
2019-08-31T12:18:24.4954739Z 
2019-08-31T12:18:24.4954807Z 1 error[E0631]: type mismatch in closure arguments
2019-08-31T12:18:24.4955106Z 2   --> $DIR/expect-fn-supply-fn.rs:30:5
2019-08-31T12:18:24.4955177Z 3    |
2019-08-31T12:18:24.4955479Z - LL |     with_closure_expecting_fn_with_free_region(|x: fn(&u32), y| {});
2019-08-31T12:18:24.4955832Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ---------------- found signature of `fn(for<'r> fn(&'r u32), _) -> _`
2019-08-31T12:18:24.4956095Z -    |     |
2019-08-31T12:18:24.4956379Z -    |     expected signature of `fn(fn(&'a u32), &i32) -> _`
2019-08-31T12:18:24.4956766Z -    |
2019-08-31T12:18:24.4956994Z - note: required by `with_closure_expecting_fn_with_free_region`
2019-08-31T12:18:24.4957425Z -    |
2019-08-31T12:18:24.4957425Z -    |
2019-08-31T12:18:24.4957507Z 12 LL | / fn with_closure_expecting_fn_with_free_region<F>(_: F)
2019-08-31T12:18:24.4957916Z 13 LL | |     where F: for<'a> FnOnce(fn(&'a u32), &i32)
2019-08-31T12:18:24.4958003Z 14 LL | | {
2019-08-31T12:18:24.4958261Z 15 LL | | }
2019-08-31T12:18:24.4958439Z -    | |_^
2019-08-31T12:18:24.4958439Z -    | |_^
2019-08-31T12:18:24.4958674Z +    | |_- required by `with_closure_expecting_fn_with_free_region`
2019-08-31T12:18:24.4958736Z + ...
2019-08-31T12:18:24.4958813Z + LL |       with_closure_expecting_fn_with_free_region(|x: fn(&u32), y| {});
2019-08-31T12:18:24.4959101Z +    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ---------------- found signature of `fn(for<'r> fn(&'r u32), _) -> _`
2019-08-31T12:18:24.4959455Z +    |       |
2019-08-31T12:18:24.4959898Z +    |       expected signature of `fn(fn(&'a u32), &i32) -> _`
2019-08-31T12:18:24.4960030Z 18 error[E0631]: type mismatch in closure arguments
2019-08-31T12:18:24.4960260Z 19   --> $DIR/expect-fn-supply-fn.rs:37:5
2019-08-31T12:18:24.4960298Z 
2019-08-31T12:18:24.4960363Z 20    |
2019-08-31T12:18:24.4960363Z 20    |
2019-08-31T12:18:24.4960777Z - LL |     with_closure_expecting_fn_with_bound_region(|x: fn(&'x u32), y| {});
2019-08-31T12:18:24.4961319Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ------------------- found signature of `fn(fn(&'x u32), _) -> _`
2019-08-31T12:18:24.4961529Z -    |     |
2019-08-31T12:18:24.4961785Z -    |     expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-08-31T12:18:24.4962224Z - note: required by `with_closure_expecting_fn_with_bound_region`
2019-08-31T12:18:24.4962445Z -   --> $DIR/expect-fn-supply-fn.rs:6:1
2019-08-31T12:18:24.4962645Z -    |
2019-08-31T12:18:24.4962645Z -    |
2019-08-31T12:18:24.4962708Z 29 LL | / fn with_closure_expecting_fn_with_bound_region<F>(_: F)
2019-08-31T12:18:24.4962959Z 30 LL | |     where F: FnOnce(fn(&u32), &i32)
2019-08-31T12:18:24.4963016Z 31 LL | | {
2019-08-31T12:18:24.4963121Z 32 LL | | }
2019-08-31T12:18:24.4963302Z -    | |_^
2019-08-31T12:18:24.4963302Z -    | |_^
2019-08-31T12:18:24.4963548Z +    | |_- required by `with_closure_expecting_fn_with_bound_region`
2019-08-31T12:18:24.4964728Z + ...
2019-08-31T12:18:24.4965138Z + LL |       with_closure_expecting_fn_with_bound_region(|x: fn(&'x u32), y| {});
2019-08-31T12:18:24.4965504Z +    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ------------------- found signature of `fn(fn(&'x u32), _) -> _`
2019-08-31T12:18:24.4965595Z +    |       |
2019-08-31T12:18:24.4965893Z +    |       expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-08-31T12:18:24.4966054Z 35 error[E0631]: type mismatch in closure arguments
2019-08-31T12:18:24.4966306Z 36   --> $DIR/expect-fn-supply-fn.rs:46:5
2019-08-31T12:18:24.4966352Z 
2019-08-31T12:18:24.4966433Z 37    |
2019-08-31T12:18:24.4966433Z 37    |
2019-08-31T12:18:24.4966702Z - LL |     with_closure_expecting_fn_with_bound_region(|x: Foo<'_>, y| {
2019-08-31T12:18:24.4967175Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ --------------- found signature of `for<'r> fn(fn(&'r u32), _) -> _`
2019-08-31T12:18:24.4967619Z -    |     |
2019-08-31T12:18:24.4968482Z -    |     expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-08-31T12:18:24.4968902Z - note: required by `with_closure_expecting_fn_with_bound_region`
2019-08-31T12:18:24.4969110Z -   --> $DIR/expect-fn-supply-fn.rs:6:1
2019-08-31T12:18:24.4969275Z -    |
2019-08-31T12:18:24.4969275Z -    |
2019-08-31T12:18:24.4969351Z 46 LL | / fn with_closure_expecting_fn_with_bound_region<F>(_: F)
2019-08-31T12:18:24.4969595Z 47 LL | |     where F: FnOnce(fn(&u32), &i32)
2019-08-31T12:18:24.4969665Z 48 LL | | {
2019-08-31T12:18:24.4969739Z 49 LL | | }
2019-08-31T12:18:24.4969932Z -    | |_^
2019-08-31T12:18:24.4969932Z -    | |_^
2019-08-31T12:18:24.4970144Z +    | |_- required by `with_closure_expecting_fn_with_bound_region`
2019-08-31T12:18:24.4970222Z + ...
2019-08-31T12:18:24.4970439Z + LL |       with_closure_expecting_fn_with_bound_region(|x: Foo<'_>, y| {
2019-08-31T12:18:24.4970743Z +    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ --------------- found signature of `for<'r> fn(fn(&'r u32), _) -> _`
2019-08-31T12:18:24.4970822Z +    |       |
2019-08-31T12:18:24.4971084Z +    |       expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-08-31T12:18:24.4971210Z 52 error: aborting due to 3 previous errors
2019-08-31T12:18:24.4971262Z 53 
2019-08-31T12:18:24.4971306Z 
2019-08-31T12:18:24.4971332Z 
2019-08-31T12:18:24.4971332Z 
2019-08-31T12:18:24.4971402Z The actual stderr differed from the expected stderr.
2019-08-31T12:18:24.4971711Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn.nll/expect-fn-supply-fn.nll.stderr
2019-08-31T12:18:24.4971970Z To update references, rerun the tests and pass the `--bless` flag
2019-08-31T12:18:24.4972417Z To only update this specific test, also pass `--test-args closure-expected-type/expect-fn-supply-fn.rs`
2019-08-31T12:18:24.4972542Z error: 1 errors occurred comparing output.
2019-08-31T12:18:24.4973031Z status: exit code: 1
2019-08-31T12:18:24.4973031Z status: exit code: 1
2019-08-31T12:18:24.4974137Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn.nll/auxiliary" "-A" "unused"
2019-08-31T12:18:24.4974684Z ------------------------------------------
2019-08-31T12:18:24.4974733Z 
2019-08-31T12:18:24.4974978Z ------------------------------------------
2019-08-31T12:18:24.4975066Z stderr:
2019-08-31T12:18:24.4975066Z stderr:
2019-08-31T12:18:24.4975310Z ------------------------------------------
2019-08-31T12:18:24.4975406Z error[E0631]: type mismatch in closure arguments
2019-08-31T12:18:24.4975854Z   --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:30:5
2019-08-31T12:18:24.4975958Z    |
2019-08-31T12:18:24.4976047Z LL | / fn with_closure_expecting_fn_with_free_region<F>(_: F)
2019-08-31T12:18:24.4976323Z LL | |     where F: for<'a> FnOnce(fn(&'a u32), &i32)
2019-08-31T12:18:24.4976574Z LL | | {
2019-08-31T12:18:24.4976840Z LL | | }
2019-08-31T12:18:24.4977257Z    | |_- required by `with_closure_expecting_fn_with_free_region`
2019-08-31T12:18:24.4977321Z ...
2019-08-31T12:18:24.4977399Z LL |       with_closure_expecting_fn_with_free_region(|x: fn(&u32), y| {});
2019-08-31T12:18:24.4977694Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ---------------- found signature of `fn(for<'r> fn(&'r u32), _) -> _`
2019-08-31T12:18:24.4977790Z    |       |
2019-08-31T12:18:24.4978090Z    |       expected signature of `fn(fn(&'a u32), &i32) -> _`
2019-08-31T12:18:24.4978214Z error[E0631]: type mismatch in closure arguments
2019-08-31T12:18:24.4978508Z   --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:37:5
2019-08-31T12:18:24.4978574Z    |
2019-08-31T12:18:24.4978574Z    |
2019-08-31T12:18:24.4978648Z LL | / fn with_closure_expecting_fn_with_bound_region<F>(_: F)
2019-08-31T12:18:24.4978711Z LL | |     where F: FnOnce(fn(&u32), &i32)
2019-08-31T12:18:24.4978782Z LL | | {
2019-08-31T12:18:24.4978831Z LL | | }
2019-08-31T12:18:24.4979077Z    | |_- required by `with_closure_expecting_fn_with_bound_region`
2019-08-31T12:18:24.4979139Z ...
2019-08-31T12:18:24.4979395Z LL |       with_closure_expecting_fn_with_bound_region(|x: fn(&'x u32), y| {});
2019-08-31T12:18:24.4979686Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ------------------- found signature of `fn(fn(&'x u32), _) -> _`
2019-08-31T12:18:24.4979779Z    |       |
2019-08-31T12:18:24.4980017Z    |       expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-08-31T12:18:24.4980138Z error[E0631]: type mismatch in closure arguments
2019-08-31T12:18:24.4983826Z   --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:46:5
2019-08-31T12:18:24.4983929Z    |
2019-08-31T12:18:24.4983929Z    |
2019-08-31T12:18:24.4984016Z LL | / fn with_closure_expecting_fn_with_bound_region<F>(_: F)
2019-08-31T12:18:24.4984093Z LL | |     where F: FnOnce(fn(&u32), &i32)
2019-08-31T12:18:24.4984177Z LL | | {
2019-08-31T12:18:24.4984234Z LL | | }
2019-08-31T12:18:24.4984559Z    | |_- required by `with_closure_expecting_fn_with_bound_region`
2019-08-31T12:18:24.4984635Z ...
2019-08-31T12:18:24.4984929Z LL |       with_closure_expecting_fn_with_bound_region(|x: Foo<'_>, y| {
2019-08-31T12:18:24.4985275Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ --------------- found signature of `for<'r> fn(fn(&'r u32), _) -> _`
2019-08-31T12:18:24.4985385Z    |       |
2019-08-31T12:18:24.4985671Z    |       expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-08-31T12:18:24.4985807Z error: aborting due to 3 previous errors
2019-08-31T12:18:24.4985850Z 
2019-08-31T12:18:24.4985911Z 
2019-08-31T12:18:24.4986157Z ------------------------------------------
2019-08-31T12:18:24.4986157Z ------------------------------------------
2019-08-31T12:18:24.4986202Z 
2019-08-31T12:18:24.4986254Z 
2019-08-31T12:18:24.4986517Z ---- [ui (nll)] ui/kindck/kindck-send-object1.rs stdout ----
2019-08-31T12:18:24.4986769Z diff of stderr:
2019-08-31T12:18:24.4986966Z 
2019-08-31T12:18:24.4987211Z 1 error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
2019-08-31T12:18:24.4987425Z 2   --> $DIR/kindck-send-object1.rs:10:5
2019-08-31T12:18:24.4987499Z 3    |
2019-08-31T12:18:24.4987695Z + LL | fn assert_send<T:Send+'static>() { }
2019-08-31T12:18:24.4987936Z +    | -------------------------------- required by `assert_send`
2019-08-31T12:18:24.4987998Z + ...
2019-08-31T12:18:24.4988208Z 4 LL |     assert_send::<&'a dyn Dummy>();
2019-08-31T12:18:24.4988478Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
2019-08-31T12:18:24.4988593Z 
2019-08-31T12:18:24.4988593Z 
2019-08-31T12:18:24.4989152Z 7    = help: the trait `std::marker::Sync` is not implemented for `(dyn Dummy + 'a)`
2019-08-31T12:18:24.4989449Z 8    = note: required because of the requirements on the impl of `std::marker::Send` for `&'a (dyn Dummy + 'a)`
2019-08-31T12:18:24.4989673Z - note: required by `assert_send`
2019-08-31T12:18:24.4989871Z -   --> $DIR/kindck-send-object1.rs:5:1
2019-08-31T12:18:24.4990054Z -    |
2019-08-31T12:18:24.4990245Z - LL | fn assert_send<T:Send+'static>() { }
2019-08-31T12:18:24.4990876Z 14 
2019-08-31T12:18:24.4990876Z 14 
2019-08-31T12:18:24.4991125Z 15 error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
2019-08-31T12:18:24.4991342Z 16   --> $DIR/kindck-send-object1.rs:29:5
2019-08-31T12:18:24.4991447Z 17    |
2019-08-31T12:18:24.4991447Z 17    |
2019-08-31T12:18:24.4991719Z + LL | fn assert_send<T:Send+'static>() { }
2019-08-31T12:18:24.4991998Z +    | -------------------------------- required by `assert_send`
2019-08-31T12:18:24.4992062Z + ...
2019-08-31T12:18:24.4992301Z 18 LL |     assert_send::<Box<dyn Dummy + 'a>>();
2019-08-31T12:18:24.4992571Z 19    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
2019-08-31T12:18:24.4992689Z 
2019-08-31T12:18:24.4992689Z 
2019-08-31T12:18:24.4992949Z 21    = help: the trait `std::marker::Send` is not implemented for `(dyn Dummy + 'a)`
2019-08-31T12:18:24.4993703Z 22    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn Dummy + 'a)>`
2019-08-31T12:18:24.4994065Z 23    = note: required because it appears within the type `std::boxed::Box<(dyn Dummy + 'a)>`
2019-08-31T12:18:24.4994314Z - note: required by `assert_send`
2019-08-31T12:18:24.4994577Z -   --> $DIR/kindck-send-object1.rs:5:1
2019-08-31T12:18:24.4994785Z -    |
2019-08-31T12:18:24.4995052Z - LL | fn assert_send<T:Send+'static>() { }
2019-08-31T12:18:24.4995381Z 29 
2019-08-31T12:18:24.4995451Z 30 error: aborting due to 2 previous errors
2019-08-31T12:18:24.4995533Z 31 
2019-08-31T12:18:24.4995569Z 
2019-08-31T12:18:24.4995569Z 
2019-08-31T12:18:24.4995604Z 
2019-08-31T12:18:24.4995688Z The actual stderr differed from the expected stderr.
2019-08-31T12:18:24.4996080Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll/kindck-send-object1.nll.stderr
2019-08-31T12:18:24.4996387Z To update references, rerun the tests and pass the `--bless` flag
2019-08-31T12:18:24.4997093Z To only update this specific test, also pass `--test-args kindck/kindck-send-object1.rs`
2019-08-31T12:18:24.4997207Z error: 1 errors occurred comparing output.
2019-08-31T12:18:24.5001352Z status: exit code: 1
2019-08-31T12:18:24.5001352Z status: exit code: 1
2019-08-31T12:18:24.5002454Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-send-object1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll/auxiliary" "-A" "unused"
2019-08-31T12:18:24.5003859Z ------------------------------------------
2019-08-31T12:18:24.5003925Z 
2019-08-31T12:18:24.5004237Z ------------------------------------------
2019-08-31T12:18:24.5004307Z stderr:
2019-08-31T12:18:24.5004307Z stderr:
2019-08-31T12:18:24.5004564Z ------------------------------------------
2019-08-31T12:18:24.5004845Z error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
2019-08-31T12:18:24.5005159Z   --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:10:5
2019-08-31T12:18:24.5005237Z    |
2019-08-31T12:18:24.5005497Z LL | fn assert_send<T:Send+'static>() { }
2019-08-31T12:18:24.5006009Z    | -------------------------------- required by `assert_send`
2019-08-31T12:18:24.5006264Z ...
2019-08-31T12:18:24.5006545Z LL |     assert_send::<&'a dyn Dummy>();
2019-08-31T12:18:24.5006875Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
2019-08-31T12:18:24.5007137Z    |
2019-08-31T12:18:24.5007374Z    = help: the trait `std::marker::Sync` is not implemented for `(dyn Dummy + 'a)`
2019-08-31T12:18:24.5007665Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&'a (dyn Dummy + 'a)`
2019-08-31T12:18:24.5007719Z 
2019-08-31T12:18:24.5008514Z error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
2019-08-31T12:18:24.5008776Z   --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:29:5
2019-08-31T12:18:24.5008856Z    |
2019-08-31T12:18:24.5009446Z LL | fn assert_send<T:Send+'static>() { }
2019-08-31T12:18:24.5009947Z    | -------------------------------- required by `assert_send`
2019-08-31T12:18:24.5010014Z ...
2019-08-31T12:18:24.5010390Z LL |     assert_send::<Box<dyn Dummy + 'a>>();
2019-08-31T12:18:24.5011754Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
2019-08-31T12:18:24.5011848Z    |
2019-08-31T12:18:24.5012085Z    = help: the trait `std::marker::Send` is not implemented for `(dyn Dummy + 'a)`
2019-08-31T12:18:24.5012381Z    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn Dummy + 'a)>`
2019-08-31T12:18:24.5012642Z    = note: required because it appears within the type `std::boxed::Box<(dyn Dummy + 'a)>`
2019-08-31T12:18:24.5012758Z error: aborting due to 2 previous errors
2019-08-31T12:18:24.5012791Z 
2019-08-31T12:18:24.5013474Z For more information about this error, try `rustc --explain E0277`.
2019-08-31T12:18:24.5013547Z 
---
2019-08-31T12:18:24.5015377Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-31T12:18:24.5015472Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-31T12:18:24.5015544Z 
2019-08-31T12:18:24.5015578Z 
2019-08-31T12:18:24.5018020Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-31T12:18:24.5018721Z 
2019-08-31T12:18:24.5018754Z 
2019-08-31T12:18:24.5018812Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-31T12:18:24.5018897Z Build completed unsuccessfully in 1:53:58
2019-08-31T12:18:24.5018897Z Build completed unsuccessfully in 1:53:58
2019-08-31T12:18:24.5066679Z == clock drift check ==
2019-08-31T12:18:24.5086462Z   local time: Sat Aug 31 12:18:24 UTC 2019
2019-08-31T12:18:24.6612921Z   network time: Sat, 31 Aug 2019 12:18:24 GMT
2019-08-31T12:18:24.6613036Z == end clock drift check ==
2019-08-31T12:18:25.5099798Z ##[error]Bash exited with code '1'.
2019-08-31T12:18:25.5156764Z ##[section]Starting: Upload CPU usage statistics
2019-08-31T12:18:25.5164446Z ==============================================================================
2019-08-31T12:18:25.5164539Z Task         : Bash
2019-08-31T12:18:25.5164626Z Description  : Run a Bash script on macOS, Linux, or Windows
