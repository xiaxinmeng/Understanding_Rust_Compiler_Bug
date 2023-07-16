
[00:39:41] ---- [ui] ui/suggestions/suggest-methods.rs stdout ----
[00:39:41] 	normalized stderr:
[00:39:41] error[E0599]: no method named `bat` found for type `Foo` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:28:7
[00:39:41]    |
[00:39:41] 28 |     f.bat(1.0);
[00:39:41]    |       ^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `bar`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `is_emtpy` found for type `std::string::String` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:31:15
[00:39:41]    |
[00:39:41] 31 |     let _ = s.is_emtpy();
[00:39:41]    |               ^^^^^^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `is_empty`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `count_eos` found for type `u32` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:35:19
[00:39:41]    |
[00:39:41] 35 |     let _ = 63u32.count_eos();
[00:39:41]    |                   ^^^^^^^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `count_zeros`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `count_o` found for type `u32` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:38:19
[00:39:41]    |
[00:39:41] 38 |     let _ = 63u32.count_o();
[00:39:41]    |                   ^^^^^^^
[00:39:41] 
[00:39:41] error: aborting due to 4 previous errors
[00:39:41] 
[00:39:41] 
[00:39:41] 
[00:39:41] expected stderr:
[00:39:41] error[E0599]: no method named `bat` found for type `Foo` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:28:7
[00:39:41]    |
[00:39:41] 28 |     f.bat(1.0);
[00:39:41]    |       ^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `bar`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `is_emtpy` found for type `std::string::String` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:31:15
[00:39:41]    |
[00:39:41] 31 |     let _ = s.is_emtpy();
[00:39:41]    |               ^^^^^^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `is_empty`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `count_eos` found for type `u32` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:34:19
[00:39:41]    |
[00:39:41] 35 |     let _ = 63u32.count_eos();
[00:39:41]    |                   ^^^^^^^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `count_ones`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `count_o` found for type `u32` in the current scope
[00:39:41]   --> $DIR/suggest-methods.rs:35:19
[00:39:41]    |
[00:39:41] 38 |     let _ = 63u32.count_o();
[00:39:41]    |                   ^^^^^^^
[00:39:41] 
[00:39:41] error: aborting due to 4 previous errors
[00:39:41] 
[00:39:41] 
[00:39:41] 
[00:39:41] diff of stderr:
[00:39:41] 
[00:39:41]  error[E0599]: no method named `bat` found for type `Foo` in the current scope
[00:39:41]    --> $DIR/suggest-methods.rs:28:7
[00:39:41]     |
[00:39:41]  28 |     f.bat(1.0);
[00:39:41]     |       ^^^
[00:39:41]     |
[00:39:41]     = help: did you mean `bar`?
[00:39:41]  
[00:39:41]  error[E0599]: no method named `is_emtpy` found for type `std::string::String` in the current scope
[00:39:41]    --> $DIR/suggest-methods.rs:31:15
[00:39:41]     |
[00:39:41]  31 |     let _ = s.is_emtpy();
[00:39:41]     |               ^^^^^^^^
[00:39:41]     |
[00:39:41]     = help: did you mean `is_empty`?
[00:39:41]  
[00:39:41]  error[E0599]: no method named `count_eos` found for type `u32` in the current scope
[00:39:41] -  --> $DIR/suggest-methods.rs:34:19
[00:39:41] +  --> $DIR/suggest-methods.rs:35:19
[00:39:41]     |
[00:39:41]  35 |     let _ = 63u32.count_eos();
[00:39:41]     |                   ^^^^^^^^^
[00:39:41]     |
[00:39:41] -   = help: did you mean `count_ones`?
[00:39:41] +   = help: did you mean `count_zeros`?
[00:39:41]  
[00:39:41]  error[E0599]: no method named `count_o` found for type `u32` in the current scope
[00:39:41] -  --> $DIR/suggest-methods.rs:35:19
[00:39:41] +  --> $DIR/suggest-methods.rs:38:19
[00:39:41]     |
[00:39:41]  38 |     let _ = 63u32.count_o();
[00:39:41]     |                   ^^^^^^^
[00:39:41]  
[00:39:41]  error: aborting due to 4 previous errors
[00:39:41]  
[00:39:41] 
[00:39:41] The actual stderr differed from the expected stderr.
[00:39:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods.stderr
[00:39:41] To update references, run this command from build directory:
[00:39:41] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'suggestions/suggest-methods.rs'
[00:39:41] 
[00:39:41] error: 1 errors occurred comparing output.
[00:39:41] status: exit code: 101
[00:39:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-methods.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods.stage2-x86_64-unknown-linux-gnu.ui.libaux" "-A" "unused"
[00:39:41] stdout:
[00:39:41] ------------------------------------------
[00:39:41] 
[00:39:41] ------------------------------------------
[00:39:41] stderr:
[00:39:41] ------------------------------------------
[00:39:41] error[E0599]: no method named `bat` found for type `Foo` in the current scope
[00:39:41]   --> /checkout/src/test/ui/suggestions/suggest-methods.rs:28:7
[00:39:41]    |
[00:39:41] 28 |     f.bat(1.0);
[00:39:41]    |       ^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `bar`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `is_emtpy` found for type `std::string::String` in the current scope
[00:39:41]   --> /checkout/src/test/ui/suggestions/suggest-methods.rs:31:15
[00:39:41]    |
[00:39:41] 31 |     let _ = s.is_emtpy();
[00:39:41]    |               ^^^^^^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `is_empty`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `count_eos` found for type `u32` in the current scope
[00:39:41]   --> /checkout/src/test/ui/suggestions/suggest-methods.rs:35:19
[00:39:41]    |
[00:39:41] 35 |     let _ = 63u32.count_eos();
[00:39:41]    |                   ^^^^^^^^^
[00:39:41]    |
[00:39:41]    = help: did you mean `count_zeros`?
[00:39:41] 
[00:39:41] error[E0599]: no method named `count_o` found for type `u32` in the current scope
[00:39:41]   --> /checkout/src/test/ui/suggestions/suggest-methods.rs:38:19
[00:39:41]    |
[00:39:41] 38 |     let _ = 63u32.count_o();
[00:39:41]    |                   ^^^^^^^
[00:39:41] 
[00:39:41] error: aborting due to 4 previous errors
[00:39:41] 
[00:39:41] 
[00:39:41] ------------------------------------------
[00:39:41] 
[00:39:41] thread '[ui] ui/suggestions/suggest-methods.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:39:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:41] 
[00:39:41] 
[00:39:41] failures:
[00:39:41]     [ui] ui/suggestions/suggest-methods.rs
[00:39:41] 
[00:39:41] test result: [31mFAILED(B[m. 400 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
