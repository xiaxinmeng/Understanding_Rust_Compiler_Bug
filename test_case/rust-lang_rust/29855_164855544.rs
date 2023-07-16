 rust
// Droppable is *NOT* Copy-able
#[rustc_mir(graphviz="foo.gv")]
fn mir() {
    let d1 = Droppable;
    drop(d1);
}
