Rust
let Foo::BarA(foo) = hi
else if let Foo::BarB(foo) { return; } // We could theoretically allow dropping the expr here and just match on the `hi` expr
else if let Foo::BarC(foo, foo2) { return; }
else if true { // Is this tail if confusing? It does *not* try to do something with the `hi` expr
    panic!()
} else {
    return;
};
