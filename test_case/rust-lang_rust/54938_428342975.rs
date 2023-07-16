plain
[00:53:03] .................................................................................................... 300/4577
[00:53:07] .................................................................................................... 400/4577
[00:53:11] .................................................................................................... 500/4577
[00:53:16] .......................i............................................................................ 600/4577
[00:53:23] ..........................F...............................................F......................... 700/4577
[00:53:33] ...................................................iiiii............................................ 900/4577
[00:53:37] .................................................................................................... 1000/4577
[00:53:40] .................................................................................................... 1100/4577
[00:53:43] .................................................................................................... 1200/4577
---
[00:54:48] ....................................i............................................................... 3000/4577
[00:54:51] ................................................................................................i.i. 3100/4577
[00:54:55] .ii................................................................................................. 3200/4577
[00:54:58] .................................................................................................... 3300/4577
[00:55:02] .........F.....................................i.................................................... 3400/4577
[00:55:08] .................................................................................................... 3600/4577
[00:55:11] .................................................................................................... 3700/4577
[00:55:15] ..........................................................................................i......... 3800/4577
[00:55:20] .................................................................................................... 3900/4577
---
[00:55:42] 
[00:55:42] ---- [ui] ui/consts/const-eval/conditional_array_execution.rs stdout ----
[00:55:42] diff of stderr:
[00:55:42] 
[00:55:42] 21 LL |     println!("{}", FOO);
[00:55:42] 23 
[00:55:42] - error[E0080]: erroneous constant used
[00:55:42] + error[E0080]: could not evaluate constant
[00:55:42] 25   --> $DIR/conditional_array_execution.rs:19:20
[00:55:42] 25   --> $DIR/conditional_array_execution.rs:19:20
[00:55:42] 26    |
[00:55:42] 27 LL |     println!("{}", FOO);
[00:55:42] 
[00:55:42] 28    |                    ^^^ referenced constant has errors
[00:55:42] - error: aborting due to 2 previous errors
[00:55:42] + error[E0080]: constant evaluation error
[00:55:42] +   --> $DIR/conditional_array_execution.rs:15:1
[00:55:42] +    |
[00:55:42] +    |
[00:55:42] + LL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
[00:55:42] +    |                   |
[00:55:42] +    |                   attempt to subtract with overflow
[00:55:42] + 
[00:55:42] + error: aborting due to 3 previous errors
[00:55:42] + error: aborting due to 3 previous errors
[00:55:42] 31 
[00:55:42] 32 For more information about this error, try `rustc --explain E0080`.
[00:55:42] 33 
[00:55:42] 
[00:55:42] 
[00:55:42] The actual stderr differed from the expected stderr.
[00:55:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/conditional_array_execution.stderr
[00:55:42] To update references, rerun the tests and pass the `--bless` flag
[00:55:42] To only update this specific test, also pass `--test-args consts/const-eval/conditional_array_execution.rs`
[00:55:42] error: 1 errors occurred comparing output.
[00:55:42] status: exit code: 1
[00:55:42] status: exit code: 1
[00:55:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary" "-A" "unused"
[00:55:42] ------------------------------------------
[00:55:42] 
[00:55:42] ------------------------------------------
[00:55:42] stderr:
[00:55:42] stderr:
[00:55:42] ------------------------------------------
[00:55:42] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":542,"byte_end":547,"line_start":15,"line_end":15,"column_start":19,"column_end":24,"is_primary":false,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":19,"highlight_end":24}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":524,"byte_end":574,"line_start":15,"line_end":15,"column_start":1,"column_end":51,"is_primary":true,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":1,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":475,"byte_end":484,"line_start":11,"line_end":11,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expaexpansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":646,"byte_end":649,"line_start":19,"line_end":19,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    println!(\"{}\", FOO);","highlight_start":20,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: referenced constant has errors\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:19:20\n   |\nLL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];\n   |                   ----- attempt to subtract with overflow\n...\nLL |     println!(\"{}\", FOO);\n   |                    ^^^\n\n"}
[00:55:42] {"message":"could not evaluate constant","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n