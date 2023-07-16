\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/str/str-idx.rs","byte_start":59,"byte_end":63,"line_start":3,"line_end":3,"column_start":17,"column_end":21,"is_primary":true,"text":[{"text":"    let c: u8 = s[4]; //~ ERROR the type `str` cannot be indexed by `{integer}`","highlight_start":17,"highlight_end":21}],"label":"slice indices are of type `usize` or ranges of `usize`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::slice::SliceIndex<str>` is not implemented for `{integer}`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `std::ops::Index<{integer}>` for `str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `{integer}: std::slice::SliceIndex<str>` is not satisfied\n  --> /checkout/src/test/ui/str/str-idx.rs:3:17\n   |\nLL |     let c: u8 = s[4]; //~ ERROR the type `str` cannot be indexed by `{integer}`\n   |                 ^^^^ slice indices are of type `usize` or ranges of `usize`\n   |\n   = help: the trait `std::slice::SliceIndex<str>` is not implemented for `{integer}`\n   = note: required because of the requirements on the impl of `std::ops::Index<{integer}>` for `str`\n\n"}
[01:06:10] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[01:06:10] 
[01:06:10] ------------------------------------------
[01:06:10] 
[01:06:10] 
[01:06:10] thread '[ui] ui/str/str-idx.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:06:10] 
[01:06:10] ---- [ui] ui/str/str-mut-idx.rs stdout ----
[01:06:10] diff of stderr:
[01:06:10] 
[01:06:10] 22    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:06:10] 23    = note: the left-hand-side of an assignment must have a statically known size
[01:06:10] 24 
[01:06:10] - error[E0277]: the type `str` cannot be mutably indexed by `usize`
[01:06:10] + error[E0277]: the trait bound `usize: std::slice::SliceIndex<str>` is not satisfied
[01:06:10] 27    |
[01:06:10] 27    |
[01:06:10] 28 LL |     s[1usize] = bot();
[01:06:10] -    |     ^^^^^^^^^ `str` cannot be mutably indexed by `usize`
[01:06:10] -    |     ^^^^^^^^^ `str` cannot be mutably indexed by `usize`
[01:06:10] +    |     ^^^^^^^^^ slice indices are of type `usize` or ranges of `usize`
[01:06:10] 30    |
[01:06:10] -    = help: the trait `std::ops::IndexMut<usize>` is not implemented for `str`
[01:06:10] +    = help: the trait `std::slice::SliceIndex<str>` is not implemented for `usize`
[01:06:10] +    = note: required because of the requirements on the impl of `std::ops::Index<usize>` for `str`
[01:06:10] 33 error: aborting due to 3 previous errors
[01:06:10] 34 
[01:06:10] 
[01:06:10] 
[01:06:10] 
[01:06:10] The actual stderr differed from the expected stderr.
[01:06:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx/str-mut-idx.stderr
[01:06:10] To update references, rerun the tests and pass the `--bless` flag
[01:06:10] To only update this specific test, also pass `--test-args str/str-mut-idx.rs`
[01:06:10] error: 1 errors occurred comparing output.
[01:06:10] status: exit code: 1
[01:06:10] status: exit code: 1
[01:06:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-mut-idx.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx/auxiliary" "-A" "unused"
[01:06:10] ------------------------------------------
[01:06:10] 
[01:06:10] ------------------------------------------
[01:06:10] stderr:
[01:06:10] stderr:
[01:06:10] ------------------------------------------
[01:06:10] {"message":"the size for values of type `str` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n