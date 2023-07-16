\n\nIf this is not an option, consider replacing the type parameter with another\ntrait object (e.g. if `T: OtherTrait`, use `on: Box<OtherTrait>`). If the number\nof types you intend to feed to this method is limited, consider manually listing\nout the methods of different types.\n\n### Method has no receiver\n\nMethods that do not take a `self` parameter can't be called since there won't be\na way to get a pointer to the method table for them.\nnger safe,\nso they are forbidden when specifying supertraits.\n\nThere's no easy fix for this, generally code will need to be refactored so that\nyou no longer need to derive from `Super<Self>`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-object-macro-matcher.rs","byte_start":581,"byte_end":602,"line_start":18,"line_end":18,"column_start":8,"column_end":29,"is_primary":true,"text":[{"text":"    m!(Copy + Send + 'static); //~ ERROR the trait `std::marker::Copy` cannot be made into an object","highlight_start":8,"highlight_end":29}],"label":"the trait `std::marker::Copy` cannot be made into an object","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait cannot require that `Self : Sized`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0038]: the trait `std::marker::Copy` cannot be made into an object\n  --> /checkout/src/test/ui/traits/trait-object-macro-matcher.rs:18:8\n   |\nLL |     m!(Copy + Send + 'static); //~ ERROR the trait `std::marker::Copy` cannot be made into an object\n   |        ^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` cannot be made into an object\n   |\n   = note: the trait cannot require that `Self : Sized`\n\n"}
[00:46:20] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:46:20] {"message":"Some errors occurred: E0038, E0224, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0038, E0224, E0277.\n"}
[00:46:20]tuple may have a dynamically sized type
[00:46:20] 58 error[E0277]: the size for values of type `X` cannot be known at compilation time
[00:46:20] +   --> $DIR/unsized6.rs:32:12
[00:46:20] +    |
[00:46:20] +    |
[00:46:20] + LL |     let y: X = *x1;
[00:46:20] +    |            ^ doesn't have a size known at compile-time
[00:46:20] +    = help: the trait `std::marker::Sized` is not implemented for `X`
[00:46:20] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:46:20] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:46:20] +    = help: consider adding a `where X: std::marker::Sized` bound
[00:46:20] +    = note: the return type of a function must have a statically known size
[00:46:20] + 
[00:46:20] + error[E0277]: the size for values of type `X` cannot be known at compilation time
[00:46:20] 60    |
[00:46:20] 60    |
[00:46:20] 61 LL |     let y: X = *x1;
[00:46:20] 92    = help: unsized locals are gated as an unstable feature
[00:46:20] 93 
[00:46:20] 94 error[E0277]: the size for values of type `X` cannot be known at compilation time
[00:46:20] +   --> $DIR/unsized6.rs:40:12
[00:46:20] +   --> $DIR/unsized6.rs:40:12
[00:46:20] +    |
[00:46:20] + LL |     let y: X = *x1;
[00:46:20] +    |            ^ doesn't have a size known at compile-time
[00:46:20] +    = help: the trait `std::marker::Sized` is not implemented for `X`
[00:46:20] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:46:20] +    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:46:20] +    = help: consideain() {\n    // we now call the method with the i32 type, which doesn't implement\n    // the Foo trait\n    some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied\n}\n