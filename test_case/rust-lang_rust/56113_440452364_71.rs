\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/two-phase-cannot-nest-mut-self-calls.rs","byte_start":908,"byte_end":919,"line_start":26,"line_end":26,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        vec.push(2);","highlight_start":9,"highlight_end":20}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-cannot-nest-mut-self-calls.rs","byte_start":889,"byte_end":892,"line_start":24,"line_end":24,"column_start":5,"column_end":8,"is_primary":false,"text":[{"text":"    vec.get({","highlight_start":5,"highlight_end":8}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-cannot-nest-m    |         first mutable borrow occurs here
[00:48:22] -    |         first borrow later used by call
[00:48:22] +    |         first borrow used by call, in later iteration of loop
[00:48:22] 9 
[00:48:22] 10 error[E0382]: use of moved value: `*f`
[00:48:22] 
[00:48:22] 
[00:48:22] 22    |         - ^ second mutable borrow occurs here
[00:48:22] 24    |         first mutable borrow occurs here
[00:48:22] -    |         first borrow later used by call
[00:48:22] -    |         first borrow later used by call
[00:48:22] +    |         first borrow used by call, in later iteration of loop
[00:48:22] 26 
[00:48:22] 27 error[E0161]: cannot move a value of type dyn std::ops::FnOnce(i32) -> i32: the size of dyn std::ops::FnOnce(i32) -> i32 cannot be statically determined
[00:48:22] 
[00:48:22] 61    |     | |
[00:48:22] 62    |     | immutable borrow occurs here
[00:48:22] 63    |     mutable borrow occurs here
[00:48:22] 63    |     mutable borrow occurs here
[00:48:22] -    |     mutable borrow later used here
[00:48:22] +    |     mutable borrow used here, in later iteration of loop
[00:48:22] 65 
[00:48:22] 66 error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
[00:48:22] 
[00:48:22] 71    |     | |
[00:48:22] 72    |     | immutable borrow occurs here
[00:48:22] 73    |     mutable borrow occurs here
[00:48:22] 73    |     mutable borrow occurs here
[00:48:22] -    |     mutable borrow later used here
[00:48:22] +    |     mutable borrow used here, in later iteration of loop
[00:48:22] 76 error: aborting due to 9 previous errors
[00:48:22] 77 
[00:48:22] 
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/two-phase-nonrecv-autoref.nll.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args borrowck/two-phase-nonrecv-autoref.rs`
[00:48:22] 
[00:48:22] error in revision `nll`: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/auxiliary" "-A" "unused"
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] ------------------------------------------
[00:48:22] stderr:
[00:48:22] stderr:
[00:48:22] ------------------------------------------
[00:48:22] {"message":"cannot borrow `*f` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n