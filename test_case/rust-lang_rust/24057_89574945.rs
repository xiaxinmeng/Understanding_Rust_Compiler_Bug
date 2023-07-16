

failures:

---- [run-pass] run-pass/overloaded-index-assoc-list.rs stdout ----

error: compilation failed!
status: exit code: 101
command: i686-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/overloaded-index-assoc-list.rs -L i686-apple-darwin/test/run-pass/ --target=i686-apple-darwin -L i686-apple-darwin/test/run-pass/overloaded-index-assoc-list.stage2-i686-apple-darwinlibaux -C prefer-dynamic -o i686-apple-darwin/test/run-pass/overloaded-index-assoc-list.stage2-i686-apple-darwin --cfg rtopt --cfg debug -O -L i686-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/overloaded-index-assoc-list.rs:38:14: 38:16 error: lifetime name `'a` shadows another lifetime name that is already in scope
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/overloaded-index-assoc-list.rs:38     fn index<'a>(&'a self, index: &K) -> &'a V {
                                                                                                                                ^~
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/overloaded-index-assoc-list.rs:35:6: 35:8 note: shadowed lifetime `'a` declared here
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/overloaded-index-assoc-list.rs:35 impl<'a, K: PartialEq + std::fmt::Debug, V:Clone> Index<&'a K> for AssociationList<K,V> {
                                                                                                                        ^~
error: aborting due to previous error

------------------------------------------

thread '[run-pass] run-pass/overloaded-index-assoc-list.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1482

