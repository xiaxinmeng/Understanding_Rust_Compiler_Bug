
let foo = Mutable::new(Some(10));
do spawn {
    let foo = foo.take();
    ...
}
