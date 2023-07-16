plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:12] 
[00:45:12] running 1548 tests
[00:45:18] ......................................................F..........................................i..
[00:45:29] ....................................................................................................
[00:45:32] ....................................................................................................
[00:45:36] ....................................................................................................
[00:45:39] ....................................................................................................
---
[00:46:35] 
[00:46:35] ---- [ui] ui/borrowck/promote-ref-mut-in-let-issue-46557.rs stdout ----
[00:46:35] diff of stderr:
[00:46:35] 
[00:46:35] 21    = note: borrowed value must be valid for the static lifetime...
[00:46:35] 22 
[00:46:35] 23 error[E0597]: borrowed value does not live long enough
[00:46:35] -    |
[00:46:35] -    |
[00:46:35] - LL |         ref mut x => x //~ ERROR
[00:46:35] -    |         ^^^^^^^^^ temporary value does not live long enough
[00:46:35] - LL |     }
[00:46:35] - LL | }
[00:46:35] -    | - temporary value only lives until here
[00:46:35] -    |
[00:46:35] -    = note: borrowed value must be valid for the static lifetime...
[00:46:35] - 
[00:46:35] - error[E0597]: borrowed value does not live long enough
[00:46:35] -    |
[00:46:35] -    |
[00:46:35] - LL |         (ref mut x,) => x, //~ ERROR
[00:46:35] -    |          ^^^^^^^^^ borrowed value does not live long enough
[00:46:35] - LL |     }
[00:46:35] - LL | }
[00:46:35] -    | - borrowed value only lives until here
[00:46:35] -    |
[00:46:35] -    = note: borrowed value must be valid for the static lifetime...
[00:46:35] - 
[00:46:35] - error[E0597]: borrowed value does not live long enough
[00:46:35] 47    |
[00:46:35] 47    |
[00:46:35] 48 LL |     &mut 1234543 //~ ERROR
[00:46:35] 52    |
[00:46:35] 52    |
[00:46:35] 53    = note: borrowed value must be valid for the static lifetime...
[00:46:35] - error: aborting due to 5 previous errors
[00:46:35] + error: aborting due to 3 previous errors
[00:46:35] 56 
[00:46:35] 57 For more information about this error, try `rustc --explain E0597`.
[00:46:35] 57 For more information about this error, try `rustc --explain E0597`.
[00:46:35] 58 
[00:46:35] 
[00:46:35] 
[00:46:35] The actual stderr differed from the expected stderr.
[00:46:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/promote-ref-mut-in-let-issue-46557/promote-ref-mut-in-let-issue-46557.stderr
[00:46:35] To update references, rerun the tests and pass the `--bless` flag
[00:46:35] To only update this specific test, also pass `--test-args borrowck/promote-ref-mut-in-let-issue-46557.rs`
[00:46:35] error: 1 errors occurred comparing output.
[00:46:35] status: exit code: 101
[00:46:35] status: exit code: 101
[00:46:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bore\ncreated. So to fix the previous example, just make the `y` lifetime greater than\nthe `x`'s one:\n\n