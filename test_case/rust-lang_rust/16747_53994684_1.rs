
<anon>:9:1: 13:2 error: internal compiler error: cannot relate bound region: ReEarlyBound(31, TypeSpace, 0, 'a) <= ReFree(26, BrNamed(syntax::ast::DefId{krate: 0u32, node: 31u32}, 'a))
<anon>:9 impl<'a, T: 'a + ListItem<'a>> Collection for List<'a, T> {
<anon>:10     fn len(&self) -> uint {
<anon>:11         0
<anon>:12     }
<anon>:13 }
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/ast_util.rs:784
