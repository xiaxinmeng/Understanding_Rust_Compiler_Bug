rust
union Foo { bar: i8 }
let Foo { bar: _ } = Foo { bar: 5 };

union Foo2 { bar: () }
let Foo2 { bar: () } = Foo2 { bar: () };
