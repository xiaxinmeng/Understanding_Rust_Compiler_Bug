rust
union Foo { bar: i8 }
let foo = Foo { bar: 5 };
match foo { Foo {
    bar: i8::MIN..=i8::MAX, // safe since this always matches
} => {} };
