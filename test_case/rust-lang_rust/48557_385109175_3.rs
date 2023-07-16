\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-48728.rs","byte_start":680,"byte_end":723,"line_start":17,"line_end":17,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"impl<T: Clone + ?Sized> Clone for Node<[T]> {","highlight_start":1,"highlight_end":44}],"label":"first implementation here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-48728.rs","byte_start":572,"byte_end":577,"line_start":14,"line_end":14,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Clone)] //~ ERROR conflicting implementations of trait `std::clone::Clone`","highlight_start":10,"highlight_end":15}],"label":"conflicting implementation for `Node<[_]>`","suggested_replacement":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issue-48728.rs","byte_start":572,"byte_end":577,"line_start":14,"line_end":14,"column_start":10,"column_end":15,"is_primary":false,"text":[{"text":"#[derive(Clone)] //~ ERROR conflicting implementations of trait `std::clone::Clone`","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null},"macro_decl_name":"#[derive(Clone)]","def_site_span":null}}],"children":[{"message":"upstream crates may add new impl of trait `std::clone::Clone` for type `[_]` in future versions","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0119]: conflicting implementations of trait `std::clone::Clone` for type `Node<[_]>`:\n  --> /checkout/src/test/ui/issue-48728.rs:14:10\n   |\nLL | #[derive(Clone)] //~ ERROR conflicting implementations of trait `std::clone::Clone`\n   |          ^^^^^ conflicting implementation for `Node<[_]>`\n...\nLL | impl<T: Clone + ?Sized> Clone for Node<[T]> {\n   | ------------------------------------------- first implementation here\n   |\n   = note: upstream crates may add new impl of trait `std::clone::Clone` for type `[_]` in future versions\n\n"}
[00:43:22] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:22] {"message":"For more information about this error, try `rustc --explain E0119`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0119`.\n"}
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] thread '[ui] ui/issue-48728.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:43:22] 
[00:43:22] 
[00:43:22] ---- [ui] ui/trivial-bounds-inconsistent-associated-functions.rs stdout ----
[00:43:22]  
[00:43:22] error: ui test compiled successfully!
[00:43:22] status: exit code: 0
[00:43:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds-inconsistent-associated-functions.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-associated-functions.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-associated-functions.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] ------------------------------------------
[00:43:22] stderr:
---
[00:43:22] 
[00:43:22] ---- [ui] ui/trivial-bounds-inconsistent-sized.rs stdout ----
[00:43:22]  diff of stderr:
[00:43:22] 
[00:43:22] - warning: Trait bound str: std::marker::Sized does not depend on any type or lifetime parameters
[00:43:22] -   --> $DIR/trivial-bounds-inconsistent-sized.rs:24:1
[00:43:22] + error[E0161]: cannot move a value of type T<A + 'static>: the size of T<A + 'static> cannot be statically determined
[00:43:22] +   --> $DIR/trivial-bounds-inconsistent-sized.rs:27:9
[00:43:22] 3    |
[00:43:22] - LL | struct S(str, str) where str: Sized;
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] -    = note: #[warn(trivial_bounds)] on by default
[00:43:22] + LL |     let x: T<A> = *(Box::new(T { x: 1 }) as Box<T<A>>);
[00:43:22] 8 
[00:43:22] 8 
[00:43:22] - warning: Trait bound T<A + 'static>: std::marker::Sized does not depend on any type or lifetime parameters
[00:43:22] -   --> $DIR/trivial-bounds-inconsistent-sized.rs:26:1
[00:43:22] + error[E0161]: cannot move a value of type T<A + 'static>: the size of T<A + 'static> cannot be statically determined
[00:43:22] +   --> $DIR/trivial-bounds-inconsistent-sized.rs:27:19
[00:43:22] 11    |
[00:43:22] - LL | / fn unsized_local() where T<A>: Sized {
[00:43:22] - LL | |     let x: T<A> = *(Box::new(T { x: 1 }) as Box<T<A>>);
[00:43:22] - LL | | }
[00:43:22] -    | |_^
[00:43:22] + LL |     let x: T<A> = *(Box::new(T { x: 1 }) as Box<T<A>>);
[00:43:22] 16 
[00:43:22] 16 
[00:43:22] - warning: Trait bound str: std::marker::Sized does not depend on any type or lifetime parameters
[00:43:22] -    |
[00:43:22] -    |
[00:43:22] - LL | / fn return_str() -> str where str: Sized {
[00:43:22] - LL | |     *"Sized".to_string().into_boxed_str()
[00:43:22] - LL | | }
[00:43:22] -   cannot move a value of type T<A + 'static>: the size of T<A + 'static> cannot be statically determined","code":{"code":"E0161","explanation":"\nA value was moved. However, its size was not known at compile time, and only\nvalues of a known size can be moved.\n\nErroneous code example:\n\n