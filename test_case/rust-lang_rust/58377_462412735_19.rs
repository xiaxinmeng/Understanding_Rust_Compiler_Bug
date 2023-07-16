\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"<::alloc::macros::vec macros>","byte_start":121,"byte_end":124,"line_start":3,"line_end":3,"column_start":7,"column_end":10,"is_primary":true,"text":[{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":7,"highlight_end":10}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<::alloc::macros::vec macros>","byte_start":217,"byte_end":240,"line_start":4,"line_end":4,"column_start":29,"column_end":52,"is_primary":false,"text":[{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":29,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs","byte_start":1424,"byte_end":1502,"line_start":28,"line_end":31,"column_start":13,"column_end":6,"is_primary":false,"text":[{"text":"    let _ = vec![","highlight_start":13,"highlight_end":18},{"text":"        Box::new(|x| (x as u8)),","highlight_start":1,"highlight_end":33},{"text":"        box |x| (x as i16 as u8),","highlight_start":1,"highlight_end":34},{"text":"    ]: Vec<Box<Fn(i32) -> _>>;","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `[std::boxed::Box<dyn std::ops::Fn(i32) -> _>]`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"all local variables must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"unsized locals are gated as an unstable feature","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `[std::boxed::Box<dyn std::ops::Fn(i32) -> _>]` cannot be known at compilation time\n  --> /checkout/src/test/ui/coercion/coerce-expect-unsized-ascribed.rs:28:13\n   |\nLL |       let _ = vec![\n   |  _____________^\nLL | |         Box::new(|x| (x as u8)),\nLL | |         box |x| (x as i16 as u8),\nLL | |     ]: Vec<Box<Fn(i32) -> _>>;\n   | |_____^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `[std::boxed::Box<dyn std::ops::Fn(i32) -> _>]`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = note: all local variables must have a statically known size\n   = help: unsized locals are gated as an unstable feature\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:24] {"message":"aborting due to 16 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 16 previous errors\n\n"}
[01:05:24] {"message":"Some errors occurred: E0277, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0308.\n"}
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] thread '[ui] ui/coercion/coerce-expect-unsized-ascribed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] thread '[ui] ui/coercion/coerce-expect-unsized-ascribed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:24] 
[01:05:24] ---- [ui] ui/elide-errors-on-mismatched-tuple.rs stdout ----
[01:05:24] diff of stderr:
[01:05:24] 
[01:05:24] 7    = note: expected type `(A, A)`
[01:05:24] 9 
[01:05:24] - error: aborting due to previous error
[01:05:24] - error: aborting due to previous error
[01:05:24] + error[E0277]: the size for values of type `[&dyn T]` cannot be known at compilation time
[01:05:24] +    |
[01:05:24] +    |
[01:05:24] + LL |     let ts: Vec<&T> = vec![&a, &b, &c];
[01:05:24] +    |
[01:05:24] +    |
[01:05:24] +    = help: the trait `std::marker::Sized` is not implemented for `[&dyn T]`
[01:05:24] +    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:05:24] +    = help: unsized locals are gated as an unstable feature
[01:05:24] +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:05:24] 11 
[01:05:24] - For more information about this error, try `rustc --explain E0308`.
---
[01:05:24] 13 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/elide-errors-on-mismatched-tuple/elide-errors-on-mismatched-tuple.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args elide-errors-on-mismatched-tuple.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/elide-errors-on-mismatched-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/elide-errors-on-mismatched-tuple/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/elide-errors-on-mismatched-tuple/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n