plain
[00:48:32] ............................................................................i.......................
[00:48:35] ....................................................................................................
[00:48:38] .............................i.i..ii................................................................
[00:48:42] ....................................................................................................
[00:48:45] ...............F.........................i..........................................................
[00:48:52] ....................................................................................................
[00:48:55] ....................................................................................................
[00:48:58] ........................................................................i...........................
[00:49:06] ....................................................................................................
---
[00:54:02] ---- [ui] ui/nll/user-annotations/patterns.rs stdout ----
[00:54:02] diff of stderr:
[00:54:02] 
[00:54:02] 40    |
[00:54:02] 41    = note: borrowed value must be valid for the static lifetime...
[00:54:02] 42 
[00:54:02] - error[E0597]: borrowed value does not live long enough
[00:54:02] + error[E0713]: temporary value borrowed for too long
[00:54:02] 45    |
[00:54:02] 45    |
[00:54:02] 46 LL |     let _: Vec<&'static String> = vec![&String::new()];
[00:54:02] 
[00:54:02] -    |                                         ^^^^^^^^^^^^^ - temporary value only lives until here
[00:54:02] +    |                                         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
[00:54:02] 48    |                                         |
[00:54:02] -    |                                         temporary value does not live long enough
[00:54:02] +    |                                         creates a temporary which is freed while still in use
[00:54:02] 50    |
[00:54:02] 51    = note: borrowed value must be valid for the static lifetime...
[00:54:02] 
[00:54:02] 
[00:54:02] - error[E0597]: borrowed value does not live long enough
[00:54:02] + error[E0713]: temporary value borrowed for too long
[00:54:02] 55    |
[00:54:02] 55    |
[00:54:02] 56 LL |     let (_, a): (Vec<&'static String>, _) = (vec![&String::new()], 44);
[00:54:02] 
[00:54:02] -    |                                                    ^^^^^^^^^^^^^      - temporary value only lives until here
[00:54:02] +    |                                                    ^^^^^^^^^^^^^      - temporary value is freed at the end of this statement
[00:54:02] 58    |                                                    |
[00:54:02] -    |                                                    temporary value does not live long enough
[00:54:02] +    |                                                    creates a temporary which is freed while still in use
[00:54:02] 60    |
[00:54:02] 61    = note: borrowed value must be valid for the static lifetime...
[00:54:02] 
[00:54:02] 
[00:54:02] - error[E0597]: borrowed value does not live long enough
[00:54:02] + error[E0713]: temporary value borrowed for too long
[00:54:02] 65    |
[00:54:02] 65    |
[00:54:02] 66 LL |     let (_a, b): (Vec<&'static String>, _) = (vec![&String::new()], 44);
[00:54:02] 
[00:54:02] -    |                                                     ^^^^^^^^^^^^^      - temporary value only lives until here
[00:54:02] +    |                                                     ^^^^^^^^^^^^^      - temporary value is freed at the end of this statement
[00:54:02] 68    |                                                     |
[00:54:02] -    |                                                     temporary value does not live long enough
[00:54:02] +    |                                                     creates a temporary which is freed while still in use
[00:54:02] 70    |
[00:54:02] 71    = note: borrowed value must be valid for the static lifetime...
[00:54:02] 
[00:54:02] 140 
[00:54:02] 141 error: aborting due to 14 previous errors
[00:54:02] 142 
[00:54:02] 142 
[00:54:02] - For more information about this error, try `rustc --explain E0597`.
[00:54:02] + Some errors occurred: E0597, E0713.
[00:54:02] + For more information about an error, try `rustc --explain E0597`.
[00:54:02] 144 
[00:54:02] 
[00:54:02] 
[00:54:02] The actual stderr differed from the expected stderr.
[00:54:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/patterns.stderr
[00:54:02] To update references, rerun the tests and pass the `--bless` flag
[00:54:02] To only update this specific test, also pass `--test-args nll/user-annotations/patterns.rs`
[00:54:02] error: 1 errors occurred comparing output.
[00:54:02] status: exit code: 1
[00:54:02] status: exit code: 1
[00:54:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/patterns.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/auxiliary" "-A" "unused"
[00:54:02] ------------------------------------------
[00:54:02] 
[00:54:02] ------------------------------------------
[00:54:02] stderr:
[00:54:02] stderr:
[00:54:02] ------------------------------------------
[00:54:02] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n