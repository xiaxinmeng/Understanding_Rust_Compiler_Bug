
---- [compile-fail] compile-fail/borrowck-loan-of-static-data-issue-27616.rs stdout ----

error: unexpected compiler error or warning: '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/compile-fail/borrowck-loan-of-static-data-issue-27616.rs:26:5: 26:23 error: cannot assign to `*s` because it is borrowed [E0506]'
status: exit code: 101
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/compile-fail/borrowck-loan-of-static-data-issue-27616.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/borrowck-loan-of-static-data-issue-27616.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/borrowck-loan-of-static-data-issue-27616.stage2-x86_64-apple-darwin --cfg rtopt -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/compile-fail/borrowck-loan-of-static-data-issue-27616.rs:26:5: 26:23 error: cannot assign to `*s` because it is borrowed [E0506]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/compile-fail/borrowck-loan-of-static-data-issue-27616.rs:26     *s = String::new(); //~ ERROR use of moved value
                                                                                                                                           ^~~~~~~~~~~~~~~~~~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/compile-fail/borrowck-loan-of-static-data-issue-27616.rs:23:38: 23:39 note: borrow of `*s` occurs here
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/compile-fail/borrowck-loan-of-static-data-issue-27616.rs:23     let alias: &'static mut String = s;
                                                                                                                                                                            ^
error: aborting due to previous error

------------------------------------------

