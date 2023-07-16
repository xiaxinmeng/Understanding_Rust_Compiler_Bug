plain
[01:34:45] normalized stderr:
[01:34:45] error[E0308]: mismatched types
[01:34:45]   --> $DIR/async-fn.rs:39:61
[01:34:45]    |
[01:34:45] 39 |     assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&waker), Poll::Ready(31));
[01:34:45]    |                                                             ^^^^^^ types differ in mutability
[01:34:45]    = note: expected type `&mut std::task::Context<'_>`
[01:34:45]               found type `&std::task::Waker`
[01:34:45] 
[01:34:45] error: aborting due to previous error
---
[01:34:45] 
[01:34:45] +error[E0308]: mismatched types
[01:34:45] +  --> $DIR/async-fn.rs:39:61
[01:34:45] +   |
[01:34:45] +39 |     assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&waker), Poll::Ready(31));
[01:34:45] +   |                                                             ^^^^^^ types differ in mutability
[01:34:45] +   = note: expected type `&mut std::task::Context<'_>`
[01:34:45] +              found type `&std::task::Waker`
[01:34:45] +
[01:34:45] +error: aborting due to previous error
[01:34:45] +error: aborting due to previous error
[01:34:45] +
[01:34:45] +For more information about this error, try `rustc --explain E0308`.
[01:34:45] +
[01:34:45] 
[01:34:45] The actual stderr differed from the expected stderr.
[01:34:45] Actual stderr saved to /tmp/compiletestVvw5QI/async-fn.stderr
[01:34:45] To update references, run this command from build directory:
[01:34:45] tests/run-pass/update-references.sh '/tmp/compiletestVvw5QI' 'async-fn.rs'
[01:34:45] error: 1 errors occurred comparing output.
[01:34:45] status: exit code: 1
[01:34:45] status: exit code: 1
[01:34:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestVvw5QI" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestVvw5QI/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestVvw5QI/async-fn.stage-id.aux" "-A" "unused"
[01:34:45] ------------------------------------------
[01:34:45] 
[01:34:45] ------------------------------------------
[01:34:45] stderr:
[01:34:45] stderr:
[01:34:45] ------------------------------------------
[01:34:45] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n