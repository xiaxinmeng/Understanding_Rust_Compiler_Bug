\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-47184.rs","byte_start":73,"byte_end":86,"line_start":4,"line_end":4,"column_start":44,"column_end":57,"is_primary":true,"text":[{"text":"    let _vec: Vec<&'static String> = vec![&String::new()];","highlight_start":44,"highlight_end":57}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"<::alloc::macros::vec macros>","byte_start":181,"byte_end":182,"line_start":3,"line_end":3,"column_start":67,"column_end":68,"is_primary":false,"text":[{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":67,"highlight_end":68}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issues/issue-47184.rs","byte_start":67,"byte_end":87,"line_start":4,"line_end":4,"column_start":38,"column_end":58,"is_primary":false,"text":[{"text":"    let _vec: Vec<&'static String> = vec![&String::new()];","highlight_start":38,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/issues/issue-47184.rs","byte_start":44,"byte_end":64,"line_start":4,"line_end":4,"column_start":15,"column_end":35,"is_primary":false,"text":[{"text":"    let _vec: Vec<&'static String> = vec![&String::new()];","highlight_start":15,"highlight_end":35}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/issues/issue-47184.rs:4:44\n   |\nLL |     let _vec: Vec<&'static String> = vec![&String::new()];\n   |               --------------------   ------^^^^^^^^^^^^^-\n   |               |                      |     |\n   |               |                      |     creates a temporary which is freed while still in use\n   |               |                      temporary value is freed at the end of this statement\n   |               type annotation requires that borrow lasts for `'static`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:24] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] 
[01:05:24] thread '[ui] ui/issues/issue-47184.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] 
[01:05:24] ---- [ui] ui/nll/user-annotations/patterns.rs stdout ----
[01:05:24] diff of stderr:
[01:05:24] 
[01:05:24] 74   --> $DIR/patterns.rs:65:41
[01:05:24] 75    |
[01:05:24] 76 LL |     let _: Vec<&'static String> = vec![&String::new()];
[01:05:24] -    |            --------------------         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
[01:05:24] -    |            |                            |
[01:05:24] -    |            |                            creates a temporary which is freed while still in use
[01:05:24] +    |            |                      |     |
[01:05:24] +    |            |                      |     |
[01:05:24] +    |            |                      |     creates a temporary which is freed while still in use
[01:05:24] +    |            |                      temporary value is freed at the end of this statement
[01:05:24] 80    |            type annotation requires that borrow lasts for `'static`
[01:05:24] +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:05:24] 81 
[01:05:24] 81 
[01:05:24] 82 error[E0716]: temporary value dropped while borrowed
[01:05:24] 
[01:05:24] 84    |
[01:05:24] 84    |
[01:05:24] 85 LL |     let (_, a): (Vec<&'static String>, _) = (vec![&String::new()], 44);
[01:05:24] -    |                 -------------------------          ^^^^^^^^^^^^^      - temporary value is freed at the end of this statement
[01:05:24] -    |                 |                                  |
[01:05:24] -    |                 |                                  creates a temporary which is freed while still in use
[01:05:24] +    |                 |                            |     |
[01:05:24] +    |                 |                            |     |
[01:05:24] +    |                 |                            |     creates a temporary which is freed while still in use
[01:05:24] +    |                 |                            temporary value is freed at the end of this statement
[01:05:24] 89    |                 type annotation requires that borrow lasts for `'static`
[01:05:24] +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:05:24] 90 
[01:05:24] 90 
[01:05:24] 91 error[E0716]: temporary value dropped while borrowed
[01:05:24] 
[01:05:24] 93    |
[01:05:24] 93    |
[01:05:24] 94 LL |     let (_a, b): (Vec<&'static String>, _) = (vec![&String::new()], 44);
[01:05:24] -    |                  -------------------------          ^^^^^^^^^^^^^      - temporary value is freed at the end of this statement
[01:05:24] -    |                  |                                  |
[01:05:24] -    |                  |heckout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n