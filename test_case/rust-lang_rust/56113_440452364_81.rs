\n\nSince `MyStruct` is a type that is not marked `Copy`, the data gets moved out\nof `x` when we set `y`. This is fundamental to Rust's ownership system: outside\nof workarounds like `Rc`, a value cannot be owned by more than one variable.\n\nSometimes we don't need to move the value. Using a reference, we can let another\nfunction borrow the value without c,"line_start":173,"line_end":173,"column_start":7,"column_end":8,"is_primary":true,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":7,"highlight_end":8}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":6019,"byte_end":6020,"line_start":173,"line_end":173,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":5,"highlight_end":6}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":6019,"byte_end":6026,"line_start":173,"line_end":173,"column_start":5,"column_end":12,"is_primary":false,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":5,"highlight_end":12}],"label":"mutable borrow used here, in later iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable\n  --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:173:7\n   |\nLL |     i[i[3]] = i[4];\n   |     --^----\n   |     | |\n   |     | immutable borrow occurs here\n   |     mutable borrow occurs here\n   |     mutable borrow used here, in later iteration of loop\n\n"}
[00:48:22] {"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}
[00:48:22] {"message":"Some errors occurred: E0161, E0382, E0499, E0502.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0161, E0382, E0499, E0502.\n"}
[00:48:22] {"message":"For more information about an error, try `rustc --explain E0161`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0161`.\n"}
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] thread '[ui] ui/borrowck/two-phase-nonrecv-autoref.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:22] 
[00:48:22] 
[00:48:22] ---- [ui] ui/borrowck/two-phase-surprise-no-conflict.rs#nll stdout ----
[00:48:22] diff of stderr:
[00:48:22] 
[00:48:22] 25 LL |     reg.register_static(Box::new(TrivialPass::new(&mut reg.sess_mut)));
[00:48:22] 26    |     --- ---------------                           ^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] +    |     |   first borrow used by call, in later iteration of loop
[00:48:22] 29    |     first mutable borrow occurs here
[00:48:22] 30 
[00:48:22] 31 error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
[00:48:22] 
[00:48:22] 34 LL |     reg.register_bound(Box::new(TrivialPass::new_mut(&mut reg.sess_mut)));
[00:48:22] 35    |     --- --------------                               ^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] +    |     |   first borrow used by call, in later iteration of loop
[00:48:22] 38    |     first mutable borrow occurs here
[00:48:22] 39 
[00:48:22] 40 error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
[00:48:22] 
[00:48:22] 43 LL |     reg.register_univ(Box::new(TrivialPass::new_mut(&mut reg.sess_mut)));
[00:48:22] 44    |     --- -------------                               ^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] +    |     |   first borrow used by call, in later iteration of loop
[00:48:22] 47    |     first mutable borrow occurs here
[00:48:22] 48 
[00:48:22] 49 error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
[00:48:22] 
[00:48:22] 52 LL |     reg.register_ref(&TrivialPass::new_mut(&mut reg.sess_mut));
[00:48:22] 53    |     --- ------------                       ^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] -    |     |   first borrow later used by call
[00:48:22] +    |     |   first borrow used by call, in later iteration of loop
[00:48:22] 56    |     first mutable borrow occurs here
[00:48:22] 57 
[00:48:22] 58 error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable
[00:48:22] 
[00:48:22] 104 LL |     reg.register_bound(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
[00:48:22] 105    |     --- --------------                               ^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
[00:48:22] 106    |     |   |
[00:48:22] -    |     |   firstmutably borrowed\n}\n