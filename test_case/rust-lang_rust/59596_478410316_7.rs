\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/ub-enum.rs","byte_start":1580,"byte_end":1674,"line_start":56,"line_end":56,"column_start":1,"column_end":95,"is_primary":true,"text":[{"text":"const BAD_ENUM_CHAR: Option<(char, char)> = Some(('x', unsafe { TransmuteChar { a: !0 }.b }));","highlight_start":1,"highlight_end":95}],"label":"type validation failed: encountered 4294967295 at .<downcast-variant(Some)>.0.1, but expected something less or equal to 1114111","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: it is undefined behavior to use this value\n  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1\n   |\nLL | const BAD_ENUM_CHAR: Option<(char, char)> = Some(('x', unsafe { TransmuteChar { a: !0 }.b }));\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 4294967295 at .<downcast-variant(Some)>.0.1, but expected something less or equal to 1114111\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[01:16:54] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] 
---
[01:16:54] diff of stderr:
[01:16:54] 
[01:16:54] 34   --> $DIR/ub-nonnull.rs:28:1
[01:16:54] 35    |
[01:16:54] 36 LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
[01:16:54] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
[01:16:54] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10"..="30
[01:16:54] 38    |
[01:16:54] 39    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[01:16:54] 
[01:16:54] 
[01:16:54] The actual stderr differed from the expected stderr.
[01:16:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.stderr
[01:16:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.stderr
[01:16:54] To update references, rerun the tests and pass the `--bless` flag
[01:16:54] To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`
[01:16:54] error: 1 errors occurred comparing output.
[01:16:54] status: exit code: 1
[01:16:54] status: exit code: 1
[01:16:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary" "-A" "unused"
[01:16:54] ------------------------------------------
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] stderr:
[01:16:54] stderr:
[01:16:54] ------------------------------------------
[01:16:54] {"message":"it is undefined behavior to use this value","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n