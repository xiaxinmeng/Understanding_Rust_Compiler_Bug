
jturner-23759:rust jturner$ git grep E0207
...
src/test/compile-fail/E0207.rs:impl<T: Default> Foo { //~ ERROR E0207
src/test/compile-fail/impl-unused-rps-in-assoc-type.rs:impl<'a> Fun for Holder { //~ ERROR E0207
src/test/compile-fail/issue-22886.rs:impl<'a> Iterator for Newtype { //~ ERROR E0207
src/test/compile-fail/issue-35139.rs:impl<'a> MethodType for MTFn { //~ ERROR E0207
