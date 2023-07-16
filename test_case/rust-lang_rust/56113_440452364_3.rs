\nfn inside_closure(x: &mut i32)me":"/checkout/src/test/ui/E0501.rs","byte_start":1187,"byte_end":1190,"line_start":34,"line_end":34,"column_start":10,"column_end":13,"is_primary":false,"text":[{"text":"    drop(bar);","highlight_start":10,"highlight_end":13}],"label":"first borrow used here, in later iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0501]: cannot borrow `*a` as immutable because previous closure requires unique access\n  --> /checkout/src/test/ui/E0501.rs:31:23\n   |\nLL |     let bar = || {\n   |               -- closure construction occurs here\nLL |         inside_closure(a)\n   |                        - first borrow occurs due to use of `a` in closure\n...\nLL |     outside_closure_2(a); //[ast]~ ERROR cannot borrow `*a` as immutable because previous closure requires unique access\n   |                       ^ borrow occurs here\n...\nLL |     drop(bar);\n   |          --- first borrow used here, in later iteration of loop\n\n"}
[00:48:22] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:22] {"message":"For more information about this error, try `rustc --explain E0501`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0501`.\n"}
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] thread '[ui] ui/E0501.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:22] 
[00:48:22] ---- [ui] ui/borrowck/borrowck-closures-two-mut.rs stdout ----
[00:48:22] diff of stderr:
[00:48:22] 
[00:48:22] 86    |                        second mutable borrow occurs here
[00:48:22] 87 LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:48:22] 88 LL |     drop((c1, c2));
[00:48:22] -    |           -- first borrow later used here
[00:48:22] +    |           -- first borrow used here, in later iteration of loop
[00:48:22] 90 
[00:48:22] 91 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:48:22] 
[00:48:22] 101    |                        second mutable borrow occurs here
[00:48:22] 101    |                        second mutable borrow occurs here
[00:48:22] 102 LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:48:22] 103 LL |     drop((c1, c2));
[00:48:22] -    |           -- first borrow later used here
[00:48:22] +    |           -- first borrow used here, in later iteration of loop
[00:48:22] 105 
[00:48:22] 106 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:48:22] 
[00:48:22] 116    |                        second mutable borrow occurs here
[00:48:22] 116    |                        second mutable borrow occurs here
[00:48:22] 117 LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:48:22] 118 LL |     drop((c1, c2));
[00:48:22] -    |           -- first borrow later used here
[00:48:22] +    |           -- first borrow used here, in later iteration of loop
[00:48:22] 120 
[00:48:22] 121 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:48:22] 
[00:48:22] 131    |                        second mutable borrow occurs here
[00:48:22] 132 ...
[00:48:22] 132 ...
[00:48:22] 133 LL |     drop((c1, c2));
[00:48:22] -    |           -- first borrow later used here
[00:48:22] +    |           -- first borrow used here, in later iteration of loop
[00:48:22] 135 
[00:48:22] 136 error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:48:22] 
[00:48:22] 146    |                        second mutable borrow occurs here
[00:48:22] 147 ...
[00:48:22] 147 ...
[00:48:22] 148 LL |     drop((c1, c2));
[00:48:22] -    |           -- first borrow later used here
[00:48:22] +    |           -- first borrow used here, in later iteration of loop
[00:48:22] 151 error: aborting due to 10 previous errors
[00:48:22] 152 
[00:48:22] 
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-two-mut/borrowck-closures-two-mut.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args borrowck/borrowck-closures-two-mut.rs`
[00:48:22] error: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testi_start":27,"highlight_end":28}],"label":"previous borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)\n  --> /checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs:24:24\n   |\nLL |     let c1 = to_fn_mut(|| x = 4);\n   |                        -- - previous borrow occurs due to use of `x` in closure\n   |                        |\n   |                        first mutable borrow occurs here\nLL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once\n   |                        ^^ - borrow occurs due to use of `x` in closure\n   |                        |\n   |                        second mutable borrow occurs here\n...\nLL | }\n   | - first borrow ends here\n\n"}
[00:48:22] {"message":"cannot borrow `x` as mutable more than once at a time (Ast)","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n