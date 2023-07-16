rs
let constructed = construct!(Bar, a = 12, b = 14);

// expansion:
let constructed = <Bar as Foo>::construct(
    <Bar as Foo>::Options { a:  12, b: 14 }
);
