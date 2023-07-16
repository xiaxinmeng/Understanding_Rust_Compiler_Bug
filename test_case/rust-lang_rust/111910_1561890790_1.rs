Rust
let Foo::BarA(foo) = hi
else match {
    Foo::BarC(foo) => panic!(),
    Foo::BarD(foo, foo2) => panic!(),
    _ => panic!(),
};
