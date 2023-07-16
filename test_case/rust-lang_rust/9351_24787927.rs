 rust
let foo = Cell::new(10);
do spawn {
    let foo = foo.take();
    ....
}
