\n\nIf the trait `Foo` was deriving from something like `Super<String>` or\n`Super<T>` (where `Foo` itself is `Foo<T>`), this is okay, because given a type\n`get_a()` will definitely return an object of that type.\n\nHowever, if it derives from `Super<Self>`, even though `Super` is object safe,\nthe method `get_a()` would return an object of unknown type when called on the\nfunction. `Self` type parameters let us make object safe traits no longer safe,\nso they are forbidden when specifying supertraits.\n\nThere's no easy fix for this, generally code will need to be refactored so that\nyou no longer need to derive from `Super<Self>`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs","byte_start":522,"byte_end":524,"line_start":28,"line_end":28,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    let z = &x as &Foo;","highlight_start":13,"highlight_end":15}],"label":"the trait `Foo` cannot be made into an object","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait cannot require that `Self : Sized`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Foo>` for `&std::boxed::Box<{integer}>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0038]: the trait `Foo` cannot be made into an object\n  --> /checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs:28:13\n   |\nLL |     let z = &x as &Foo;\n   |             ^^ the trait `Foo` cannot be made into an object\n   |\n   = note: the trait cannot require that `Self : Sized`\n   = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Foo>` for `&std::boxed::Box<{integer}>`\n\n"}
[01:16:54] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:16:54] {"message":"Some errors occurred: E0038, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0038, E0277.\n"}
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] 
[01:16:54] 
[01:16:54] thread '[ui] ui/kindck/kindck-inherited-copy-bound.rs#curr' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:16:54] 
[01:16:54] ---- [ui] ui/kindck/kindck-inherited-copy-bound.rs#object_safe_for_dispatch stdout ----
[01:16:54] diff of stderr:
[01:16:54] 
[01:16:54] 
[01:16:54] 1 error[E0277]: the trait bound `std::boxed::Box<{integer}>: std::marker::Copy` is not satisfied
[01:16:54] 2   --> $DIR/kindck-inherited-copy-bound.rs:21:5
[01:16:54] 3    |
[01:16:54] - LL |     take_param(&x);
[01:16:54] + LL |     take_param(&x); //[curr]~ ERROR E0277
[01:16:54] 5    |     ^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::boxed::Box<{integer}>`
[01:16:54] 6    |
[01:16:54] 7    = note: required because of the requirements on the impl of `Foo` for `std::boxed::Box<{integer}>`
[01:16:54] 
[01:16:54] The actual stderr differed from the expected stderr.
[01:16:54] The actual stderr differed from the expected stderr.
[01:16:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.object_safe_for_dispatch/kindck-inherited-copy-bound.object_safe_for_dispatch.stderr
[01:16:54] To update references, rerun the tests and pass the `--bless` flag
[01:16:54] To only update this specific test, also pass `--test-args kindck/kindck-inherited-copy-bound.rs`
[01:16:54] 
[01:16:54] error in revision `object_safe_for_dispatch`: 1 errors occurred comparing output.
[01:16:54] status: exit code: 1
[01:16:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "object_safe_for_dispatch" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.object_safe_for_dispatch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.object_safe_for_dispatch/auxiliary" "-A" "unused"
[01:16:54] ------------------------------------------
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] stderr:
[01:16:54] stderr:
[01:16:54] ------------------------------------------
[01:16:54] {"message":"the trait bound `std::boxed::Box<{integer}>: std::marker::Copy` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n