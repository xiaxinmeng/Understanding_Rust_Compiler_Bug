
src/lib.rs:20:19: 20:21 error: lifetime name `'a` shadows a lifetime name that is already in scope [E0496]
src/lib.rs:20     fn node_label<'a>(&self, i: &Nd) -> dot::LabelText<'a> {
                                ^~
src/lib.rs:20:19: 20:21 help: run `rustc --explain E0496` to see a detailed explanation
src/lib.rs:13:6: 13:8 note: shadowed lifetime `'a` declared here
src/lib.rs:13 impl<'a> dot::Labeller<'a, Nd, Ed> for Graph<'a> {
                   ^~
