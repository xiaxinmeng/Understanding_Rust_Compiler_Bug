\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-46471-1.rs","byte_start":588,"byte_end"}
---
[00:42:43] - error[E0597]: `x` does not live long enough (Ast)
[00:42:43] -   --> $DIR/issue-46471.rs:15:6
[00:42:43] + error[E0597]: `x` does not live long enough (Mir)
[00:42:43] +   --> $DIR/issue-46471.rs:15:5
[00:42:43] 3    |
[00:42:43] 4 LL |     &x
[00:42:43] -    |      ^ borrowed value does not live long enough
[00:42:43] +    |     ^^ borrowed value does not live long enough
[00:42:43] 6 ...
[00:42:43] 7 LL | }
[00:42:43] 8    | - borrowed value only lives until here
[00:42:43]
[00:42:43] 9    |
[00:42:43] 10    = note: borrowed value must be valid for the static lifetime...
[00:42:43] 11
[00:42:43] - error[E0597]: `x` does not live long enough (Mir)
[00:42:43] -   --> $DIR/issue-46471.rs:15:5
[00:42:43] + error[E0597]: `x` does not live long enough (Ast)
[00:42:43] +   --> $DIR/issue-46471.rs:15:6
[00:42:43] 14    |
[00:42:43] 15 LL |     &x
[00:42:43] -    |     ^^ borrowed value does not live long enough
[00:42:43] +    |      ^ borrowed value does not live long enough
[00:42:43] 17 ...
[00:42:43] 18 LL | }
[00:42:43] 19    | - borrowed value only lives until here
[00:42:43]
[00:42:43]
[00:42:43] The actual stderr differed from the expected stderr.
[00:42:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-46471.stderr
[00:42:43] To update references, run this command from build directory:
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-46471.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 101
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-46471.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-46471.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "emit-end-regions" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-46471.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:42:43] {"message":"`x` does not live long enough (Mir)","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n