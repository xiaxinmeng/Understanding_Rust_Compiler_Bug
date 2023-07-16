
8 |     takes_ref(returns_result()?);
  |               ^^^^^^^^^^^^^^^^^
  |               |
  |               expected &Foo, found struct `Foo`
  |               help: try wrapping with a success variant: `Ok(returns_result()?)`
  |
  = note: expected type `&Foo`
             found type `Foo`
