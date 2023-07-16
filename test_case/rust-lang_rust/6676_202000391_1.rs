 rust
type __StaticAssert =
    (((
       [i8; 0 - ((false == (1 > 0)) as usize)],
       [i8; 0 - ((false == (2 > 1)) as usize)]),
      [i8; 0 - ((false == (3 > 2)) as usize)]),
     [i8; 0 - ((false == (3 > 1)) as usize)]);
