\n\nThis will fail because the compiler does not know which instance of `Foo` to\ncall `bar` on. Change `Foo::bar()` to `Foo::<T>::bar()` to resolve the error.\n"},"level":"error","spans":[{"file_name":"<::alloc::macros::vec macros>","byte_start":127,"byte_end":144,"line_start":3,"line_end":3,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":13,"highlight_end":30}],"label":"cannot infer type","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec.rs","byte_start":24,"byte_end":30,"line_start":2,"line_end":2,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let x = vec![];","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec.rs","byte_start":20,"byte_end":21,"line_start":2,"line_end":2,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"    let x = vec![];","highlight_start":9,"highlight_end":10}],"label":"consider giving `x` a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0282]: type annotations needed\n  --> /checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec.rs:2:13\n   |\nLL |     let x = vec![];\n   |         -   ^^^^^^ cannot infer type\n   |         |\n   |         consider giving `x` a type\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:24] {"message":"For more information about this error, try `rustc --explain E0282`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0282`.\n"}
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] 
[01:05:24] thread '[ui] ui/type/type-check/cannot_infer_local_or_vec.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] 
[01:05:24] ---- [ui] ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs stdout ----
[01:05:24] diff of stderr:
[01:05:24] 
[01:05:24] 2   --> $DIR/cannot_infer_local_or_vec_in_tuples.rs:2:18
[01:05:24] 3    |
[01:05:24] 4 LL |     let (x, ) = (vec![], );
[01:05:24] -    |         -----    ^^^^^^ cannot infer type for `T`
[01:05:24] 6    |         |
[01:05:24] 7    |         consider giving the pattern a type
[01:05:24] 8    |
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/cannot_infer_local_or_vec_in_tuples.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args type/type-check/cannot_infer_local_or_vec_in_tuples.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"type annotations needed","code":{"code":"E0282","explanation":"\nThis error indicates that type inference did not result in one unique possible\ntype, and extra information is required. In most cases this can be provided\nby adding a type annotation. Sometimes you need to specify a generic type\nparameter manually.\n\nA common example is the `collect` method on `Iterator`. It has a generic type\nparameter with a `FromIterator` bound, which for a `char` iterator is\nimplemented by `Vec` and `String` among others. Consider the following snippet\nthat reverses the characters of a string:\n\n