rust
#[my_funky_attr]
enum Foo { pub Bar }

// ...expands to the following because `my_funky_attr` is funky:

struct Bar;
enum Foo { Bar(Bar) }
