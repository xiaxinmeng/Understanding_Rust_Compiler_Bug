\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-sized.rs","byte_start":739,"byte_end":740,"line_start":27,"line_end":27,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: T<A> = *(Box::new(T { x: 1 }) as Box<T<A>>);","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0161]: cannot move a value of type T<A + 'static>: the size of T<A + 'static> cannot be statically determined\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent-sized.rs:27:9\n   |\nLL |     let x: T<A> = *(Box::new(T { x: 1 }) as Box<T<A>>);\n   |       0:43:22] 
[00:43:22] stderr:
[00:43:22] ------------------------------------------
[00:43:22] ------------------------------------------
[00:43:22] {"message":"Trait bound std::string::String: std::marker::Copy does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-copy.rs","byte_start":587,"byte_end":696,"line_start":16,"line_end":21,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn copy_string(t: String) -> String where String: Copy {","highlight_start":1,"highlight_end":57},{"text":"    is_copy(&t);","highlight_start":1,"highlight_end":17},{"text":"    let x = t;","highlight_start":1,"highlight_end":15},{"text":"    drop(t);","highlight_start":1,"highlight_end":13},{"text":"    t","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(trivial_bounds)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: Trait bound std::string::String: std::marker::Copy does not depend on any type or lifetime parameters\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent-copy.rs:16:1\n   |\nLL | / fn copy_string(t: String) -> String where String: Copy {\nLL | |     is_copy(&t);\nLL | |     let x = t;\nLL | |     drop(t);\nLL | |     t\nLL | | }\n   | |_^\n   |\n   = note: #[warn(trivial_bounds)] on by default\n\n"}
[00:43:22] {"message":"Trait bound std::string::String: std::marker::Copy does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-copy.rs","byte_start":698,"byte_end":768,"line_start":23,"line_end":25,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn copy_out_string(t: &String) -> String where String: Copy {","highlight_start":1,"highlight_end":62},{"text":"    *t","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: Trait bound std::string::String: std::marker::Copy does not depend on any type or lifetime parameters\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent-copy.rs:23:1\n   |\nLL | / fn copy_out_string(t: &String) -> String where String: Copy {\nLL | |     *t\nLL | | }\n   | |_^\n\n"}
[00:43:22] {"message":"Trait bound std::string::String: std::marker::Copy does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-copy.rs","byte_start":770,"byte_end":862,"line_start":27,"line_end":30,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn copy_string_with_param<T>(x: String) where String: Copy {","highlight_start":1,"highlight_end":61},{"text":"    let y = x;","highlight_start":1,"highlight_end":15},{"text":"    let z = x;","highlight_start":1,"highlight_end":15},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: Trait bound std::string::String: std::marker::Copy does not depend on any type or lifetime parameters\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent-copy.rs:27:1\n   |\nLL | / fn copy_string_with_param<T>(x: String) where String: Copy {\nLL | |     let y = x;\nLL | |     let z = x;\nLL | | }\n   | |_^\n\n"}
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] thread '[ui] ui/trivial-bounds-inconsistent-copy.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:43:22] 
[00:43:22] 
[00:43:22] ---- [ui] ui/trivial-bounds-inconsistent-well-formed.rs stdout ----
[00:43:22]  
[00:43:22] error: ui test compiled successfully!
[00:43:22] status: exit code: 0
[00:43:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds-inconsistent-well-formed.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-well-formed.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-well-formed.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] ------------------------------------------
[00:43:22] stderr:
[00:43:22] stderr:
[00:43:22] ------------------------------------------
[00:43:22] {"message":"Trait bound std::vec::Vec<str>: std::fmt::Debug does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-well-formed.rs","byte_start":615,"byte_end":713,"line_start":17,"line_end":20,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn foo() where Vec<str>: Debug, str: Copy {","highlight_start":1,"highlight_end":48},{"text":"    let x = vec![*\"1\"];","highlight_start":1,"highlight_end":24},{"text":"    println!(\"{:?}\", x);","highlight_start":1,"highlight_end":25},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(trivial_bounds)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: Trait bound std::vec::Vec<str>: std::fmt::Debug does not depend on any type or lifetime parameters\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent-well-formed.rs:17:1\n   |\nLL | / pub fn foo() where Vec<str>: Debug, str: Copy {\nLL | |     let x = vec![*\"1\"];\nLL | |     println!(\"{:?}\", x);\nLL | | }\n   | |_^\n   |\n   = note: #[warn(trivial_bounds)] on by default\n\n"}
[00:43:22] {"message":"Trait bound str: std::marker::Copy does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-well-formed.rs","byte_start":615,"00:43:22] -    = note: #[warn(trivial_bounds)] on by default
[00:43:22] + LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
[00:43:22] 8 
[00:43:22] 8 
[00:43:22] - warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
[00:43:22] -   --> $DIR/trivial-bounds-inconsistent.rs:26:1
[00:43:22] + error[E0161]: cannot move a value of type Dst<A + 'static>: the size of Dst<A + 'static> cannot be statically determined
[00:43:22] +   --> $DIR/trivial-bounds-inconsistent.rs:61:21
[00:43:22] 11    |
[00:43:22] - LL | struct S where i32: Foo;
[00:43:22] -    | ^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:22] + LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
[00:43:22] 14 
[00:43:22] 14 
[00:43:22] - warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | trait T where i32: Foo {}
[00:43:22] + error: aborting due to 2 previous errors
[00:43:22] 20 
[00:43:22] 20 
[00:43:22] - warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | union U where i32: Foo { f: i32 }
[00:43:22] - 
[00:43:22] - 
[00:43:22] - warning: where clauses are not enforced in type aliases
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | type Y where i32: Foo = ();nconsistent.rs:58:1
[00:43:22] -    |
[00:43:22] - LL | struct TwoStrs(str, str) where str: Sized;
[00:43:22] - 
[00:43:22] - 
[00:43:22] - warning: Trait bound Dst<A + 'static>: std::marker::Sized does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | / fn unsized_local() where Dst<A>: Sized {
[00:43:22] - LL | |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
[00:43:22] - LL | | }
[00:43:22] - 
[00:43:22] - 
[00:43:22] - warning: Trait bound str: std::marker::Sized does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | / fn return_str() -> str where str: Sized {
[00:43:22] - LL | |     *"Sized".to_string().into_boxed_str()
[00:43:22] - LL | | }
[00:43:22] - 
[00:43:22] - 
[00:43:22] - warning: Trait bound std::string::String: std::ops::Neg does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | / fn use_op(s: String) -> String where String: ::std::ops::Neg<Output=String> {
[00:43:22] - LL | |     -s
[00:43:22] - LL | | }
[00:43:22] - 
[00:43:22] - 
[00:43:22] - warning: Trait bound i32: std::iter::Iterator does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | / fn use_for() where i32: Iterator {
[00:43:22] - LL | |     for _ in 2i32 {}
[00:43:22] - LL | | }
[00:43:22] - 
[00:43:22] + For more information about this error, try `rustc --explain E0161`.
[00:43:22] 104 
[00:43:22] 
---
[00:43:22] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'trivial-bounds-inconsistent.rs'
[00:43:22] 
[00:43:22] error: 1 errors occurred comparing output.
[00:43:22] status: exit code: 101
[00:43:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds-inconsistent.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] ------------------------------------------
[00:43:22] stderr:
[00:43:22] stderr:
[00:43:22] ------------------------------------------
[00:43:22] {"message":"cannot move a value of type Dst<A + 'static>: the size of value of type Dst<A + 'static>: the size of Dst<A + 'static> cannot be statically determined","code":{"code":"E0161","explanation":"\nA value was moved. However, its size was not known at compile time, and only\nvalues of a known size can be moved.\n\nErroneous code example:\n\n