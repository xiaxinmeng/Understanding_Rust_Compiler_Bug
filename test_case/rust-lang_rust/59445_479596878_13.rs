\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/bad/bad-sized.rs","byte_start":64,"byte_end":72,"line_start":4,"line_end":4,"column_start":33,"column_end":41,"is_primary":true,"text":[{"text":"    let x: Vec<Trait + Sized> = Vec::new();","highlight_start":33,"highlight_end":41}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `dyn Trait`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required by `std::vec::Vec::<T>::new`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time\n  --> /checkout/src/test/ui/bad/bad-sized.rs:4:33\n   |\nLL |     let x: Vec<Trait + Sized> = Vec::new();\n   |                                 ^^^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `dyn Trait`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = note: required by `std::vec::Vec::<T>::new`\n\n"}
[01:15:35] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:15:35] {"message":"Some errors occurred: E0225, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0225, E0277.\n"}
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] thread '[ui] ui/bad/bad-sized.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] thread '[ui] ui/bad/bad-sized.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] 
[01:15:35] ---- [ui] ui/error-codes/E0225.rs stdout ----
[01:15:35] diff of stderr:
[01:15:35] 
[01:15:35] 2   --> $DIR/E0225.rs:6:32
[01:15:35] 3    |
[01:15:35] 4 LL |     let _: Box<std::io::Read + std::io::Write>;
[01:15:35] -    |                                ^^^^^^^^^^^^^^ non-auto additional trait
[01:15:35] +    |                -------------   ^^^^^^^^^^^^^^ additional non-auto trait
[01:15:35] +    |                first non-auto trait
[01:15:35] 6 
[01:15:35] 6 
[01:15:35] 7 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:15:35] 8   --> $DIR/E0225.rs:8:16
[01:15:35] 9    |
[01:15:35] 9    |
[01:15:35] 10 LL | trait Foo = std::io::Read + std::io::Write;
[01:15:35] -    |                             -------------- non-auto additional trait
[01:15:35] +    |             -------------   -------------- additional non-auto trait
[01:15:35] +    |             first non-auto trait
[01:15:35] 12 ...
[01:15:35] 12 ...
[01:15:35] 13 LL |     let _: Box<Foo>;
[01:15:35] 
[01:15:35] 
[01:15:35] The actual stderr differed from the expected stderr.
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/E0225.stderr
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/E0225.stderr
[01:15:35] To update references, rerun the tests and pass the `--bless` flag
[01:15:35] To only update this specific test, also pass `--test-args error-codes/E0225.rs`
[01:15:35] error: 1 errors occurred comparing output.
[01:15:35] status: exit code: 1
[01:15:35] status: exit code: 1
[01:15:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0225.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/auxiliary" "-A" "unused"
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] stderr:
[01:15:35] stderr:
[01:15:35] ------------------------------------------
[01:15:35] {"message":"only auto traits can be used as additional traits in a trait object","code":{"code":"E0225","explanation":"\nYou attempted to use multiple types as bounds for a closure or trait object.\nRust does not currently support this. A simple example that causes this error:\n\n