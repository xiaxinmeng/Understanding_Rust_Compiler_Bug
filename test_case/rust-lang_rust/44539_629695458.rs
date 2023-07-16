
error[E0283]: type annotations needed
  --> file13.rs:12:16
   |
4  |     const BAR: u8 = 32;
   |     ------------------- required by `Foo::BAR`
...
12 |     take_debug(Foo::BAR);
   |                ^^^^^^^^
   |                |
   |                cannot infer type
   |                help: use the fully qualified path to an implementation: `<Type as Foo>::BAR`
   |
   = note: cannot satisfy `_: Foo`
   = note: associated constants cannot be accessed directly on a `trait`, they can only be accessed through a specific `impl`
