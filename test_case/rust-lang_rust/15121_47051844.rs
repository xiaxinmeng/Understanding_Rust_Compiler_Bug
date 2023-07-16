
$ rustdoc --test src/doc/tutorial.md

running 161 tests
... /* snip */

failures:

---- Control_structures_7 stdout ----
    <anon>:5:9: 5:15 error: refutable pattern in local binding: `(_, _)` not covered
    <anon>:5     let (a, 2) = (1, 2);
                     ^~~~~~
    error: aborting due to previous error
    task 'Control_structures_7' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:127



failures:
    Control_structures_7

test result: FAILED. 133 passed; 1 failed; 27 ignored; 0 measured

task '<main>' failed at 'Some tests failed', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libtest/lib.rs:243
