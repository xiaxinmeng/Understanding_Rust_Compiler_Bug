plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:39:12] 
[00:39:12] running 1548 tests
[00:39:18] .....................................................F...........................................i..
[00:39:24] ...........................................F.................i......................................
[00:39:31] ....................................................................................................
[00:39:35] ....................................................................................................
[00:39:38] ....................................................................................................
[00:39:43] ....................................................................................................
---
[00:40:29] 
[00:40:29] ---- [ui] ui/borrowck/promote-ref-mut-in-let-issue-46557.rs stdout ----
[00:40:29] diff of stderr:
[00:40:29] 
[00:40:29] 21    = note: borrowed value must be valid for the static lifetime...
[00:40:29] 22 
[00:40:29] 23 error[E0597]: borrowed value does not live long enough
[00:40:29] -    |
[00:40:29] -    |
[00:40:29] - LL |         ref mut x => x //~ ERROR
[00:40:29] -    |         ^^^^^^^^^ temporary value does not live long enough
[00:40:29] - LL |     }
[00:40:29] - LL | }
[00:40:29] -    | - temporary value only lives until here
[00:40:29] -    |
[00:40:29] -    = note: borrowed value must be valid for the static lifetime...
[00:40:29] - 
[00:40:29] - error[E0597]: borrowed value does not live long enough
[00:40:29] -    |
[00:40:29] -    |
[00:40:29] - LL |         (ref mut x,) => x, //~ ERROR
[00:40:29] -    |          ^^^^^^^^^ borrowed value does not live long enough
[00:40:29] - LL |     }
[00:40:29] - LL | }
[00:40:29] -    | - borrowed value only lives until here
[00:40:29] -    |
[00:40:29] -    = note: borrowed value must be valid for the static lifetime...
[00:40:29] - 
[00:40:29] - error[E0597]: borrowed value does not live long enough
[00:40:29] 47    |
[00:40:29] 47    |
[00:40:29] 48 LL |     &mut 1234543 //~ ERROR
[00:40:29] 52    |
[00:40:29] 52    |
[00:40:29] 53    = note: borrowed value must be valid for the static lifetime...
[00:40:29] - error: aborting due to 5 previous errors
[00:40:29] + error: aborting due to 3 previous errors
[00:40:29] 56 
[00:40:29] 57 For more information about this error, try `rustc --explain E0597`.
[00:40:29] 57 For more information about this error, try `rustc --explain E0597`.
[00:40:29] 58 
[00:40:29] 
[00:40:29] 
[00:40:29] The actual stderr differed from the expected stderr.
[00:40:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/promote-ref-mut-in-let-issue-46557/promote-ref-mut-in-let-issue-46557.stderr
[00:40:29] To update references, rerun the tests and pass the `--bless` flag
[00:40:29] To only update this specific test, also pass `--test-args borrowck/promote-ref-mut-in-let-issue-46557.rs`
[00:40:29] error: 1 errors occurred comparing output.
[00:40:29] status: exit code: 101
[00:40:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:40:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:40:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/promote-ref-mut-in-let-issue-46557/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/promote-ref-mut-in-let-issue-46557/auxiliary" "-A" "unused"
[00:40:29] ------------------------------------------
[00:40:29] 
[00:40:29] ------------------------------------------
[00:40:29] stderr:
[00:40:29] stderr:
[00:40:29] ------------------------------------------
[00:40:29] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n