\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/mismatched_types/closure-arg-count.rs","byte_start":631,"byte_end":638,"line_start":17,"line_end":17,"column_start":15,"column_end":22,"is_primary":true,"text":[{"text":"    [1, 2, 3].sort_by(|tuple| panic!());","highlight_start":15,"highlight_end":22}],"label":"expected closure that takes 2 arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/mismatched_types/closure-arg-count.rs","byte_start":639,"byte_end":646,"line_start":17,"line_end":17,"column_start":23,"column_end":30,"is_primary":false,"text":[{"text":"    [1, 2, 3].sort_by(|tuple| panic!());","highlight_start":23,"highlight_end":30}],"label":"takes 1 argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0593]: closure is expected to take 2 arguments, but it takes 1 argument\n  --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:17:15\n   |\nLL |     [1, 2, 3].sort_by(|tuple| panic!());\n   |               ^^^^^^^ ------- takes 1 argument\n   |               |\n   |   , found enum variant `Z::Unit`
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |             Unit,
[01:05:22] -    |             ---- `Z::Unit` defined here
[01:05:22] - ...
[01:05:22] - LL |         let _ = Z::Unit();
[01:05:22] -    |                 ^^^^^^^^^ not a function
[01:05:22] - help: `Z::Unit` is a unit variant, you need to write it without the parenthesis
[01:05:22] -    |
[01:05:22] - LL |         let _ = Z::Unit;
[01:05:22] - 
[01:05:22] - error[E0308]: mismatched types
[01:05:22] -   --> $DIR/privacy-enum-ctor.rs:53:16
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     let _: E = m::E::Fn;
[01:05:22] -    |                ^^^^^^^^ expected enum `m::E`, found fn item
[01:05:22] -    |
[01:05:22] -    = note: expected type `m::E`
[01:05:22] -               found type `fn(u8) -> m::E {m::E::Fn}`
[01:05:22] - 
[01:05:22] - error[E0618]: expected function, found enum variant `m::E::Unit`
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |         Unit,
[01:05:22] -    |         ---- `m::E::Unit` defined here
[01:05:22] - ...
[01:05:22] - LL |     let _: E = m::E::Unit();
[01:05:22] -    |                ^^^^^^^^^^^^ not a function
[01:05:22] - help: `m::E::Unit` is a unit variant, you need to write it without the parenthesis
[01:05:22] -    |
[01:05:22] - LL |     let _: E = m::E::Unit;
[01:05:22] - 
[01:05:22] - error[E0308]: mismatched types
[01:05:22] -   --> $DIR/privacy-enum-ctor.rs:61:16
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     let _: E = E::Fn;
[01:05:22] -    |                ^^^^^ expected enum `m::E`, found fn item
[01:05:22] -    |
[01:05:22] -    = note: expected type `m::E`
[01:05:22] -               found type `fn(u8) -> m::E {m::E::Fn}`
[01:05:22] - 
[01:05:22] - error[E0618]: expected function, found enum variant `E::Unit`
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |         Unit,
[01:05:22] -    |         ---- `E::Unit` defined here
[01:05:22] - ...
[01:05:22] - LL |     let _: E = E::Unit();
[01:05:22] -    |                ^^^^^^^^^ not a function
[01:05:22] - help: `E::Unit` is a unit variant, you need to write it without the parenthesis
[01:05:22] -    |
[01:05:22] - LL |     let _: E = E::Unit;
[01:05:22] - 
[01:05:22] - error: aborting due to 23 previous errors
[01:05:22] + error: aborting due to 18 previous errors
[01:05:22] 225 
[01:05:22] 225 
[01:05:22] 226 Some errors occurred: E0308, E0412, E0423, E0603, E0618.
[01:05:22] 227 For more information about an error, try `rustc --explain E0308`.
[01:05:22] 
[01:05:22] 
[01:05:22] The actual stderr differed from the expected stderr.
[01:05:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/privacy-enum-ctor.stderr
[01:05:22] To update references, rerun the tests and pass the `--bless` flag
[01:05:22] To only update this specific test, also pass `--test-args resolve/privacy-enum-ctor.rs`
[01:05:22] error: 1 errors occurred comparing output.
[01:05:22] status: exit code: 101
[01:05:22] status: exit code: 101
[01:05:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-enum-ctor.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/auxiliary" "-A" "unused"
[01:05:22] ------------------------------------------
[01:05:22] 
[01:05:22] ------------------------------------------
[01:05:22] stderr:
[01:05:22] stderr:
[01:05:22] ------------------------------------------
[01:05:22] {"message":"expected value, found enum `n::Z`","code":{"code":"E0423","explanation":"\nA `struct` variant name was used like a function name.\n\nErroneous code example:\n\n