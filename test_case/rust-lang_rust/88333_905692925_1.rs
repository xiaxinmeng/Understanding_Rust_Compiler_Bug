
error[E0282]: type annotations needed
   --> src/index.rs:658:14
    |
658 |             .map(|entry| entry.apath.into()).next().is_none());
    |              ^^^         ------------------ this method call resolves to `T`
    |              |
    |              cannot infer type for type parameter `B` declared on the associated function `map`

error: aborting due to previous error; 4 warnings emitted

