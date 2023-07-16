
compile_and_link: x86_64-unknown-freebsd/stage2/lib/rustlib/x86_64-unknown-freebsd/lib/librun_pass_stage2.so
warning: ignoring specified output filename for library.
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:16:5: 16:10 error: unresolved import. maybe a missing `extern mod extra`?
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:16 use extra::json;
                                                                                                     ^~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:16:5: 16:16 error: failed to resolve import `extra::json`
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:16 use extra::json;
                                                                                                     ^~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:17:5: 17:14 error: unresolved import. maybe a missing `extern mod serialize`?
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:17 use serialize::Decodable;
                                                                                                     ^~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:17:5: 17:25 error: failed to resolve import `serialize::Decodable`
/home/rustbuild/src/rust-buildbot/slave/auto-bsd-64-opt/build/src/test/run-pass/issue-4036.rs:17 use serialize::Decodable;
                                                                                                     ^~~~~~~~~~~~~~~~~~~~
error: aborting due to 4 previous errors
gmake: *** [x86_64-unknown-freebsd/stage2/lib/rustlib/x86_64-unknown-freebsd/lib/librun_pass_stage2.so] Error 101
program finished with exit code 2
