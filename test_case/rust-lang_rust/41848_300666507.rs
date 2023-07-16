
let _foo = Foo {
    one: 100,
    ..Foo::default(),
    ..Foo::new(),
};
