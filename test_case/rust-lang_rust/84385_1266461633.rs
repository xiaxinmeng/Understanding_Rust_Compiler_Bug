plain
   Compiling either v1.6.0
   Compiling bitflags v1.3.2
   Compiling itoa v1.0.2
   Compiling parking_lot v0.11.2
error[E0309]: the parameter type `E` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/nom-7.1.0/src/multi/mod.rs:582:3
    |
582 | /   move |i: I| {
583 | |     let mut input = i.clone();
584 | |
585 | |     for elem in buf.iter_mut() {
...   |
601 | |     Ok((input, ()))
602 | |   }
    | |___^ ...so that the type `E` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
    |
580 |   E: ParseError<I> + 'a,

error[E0309]: the parameter type `I` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/nom-7.1.0/src/multi/mod.rs:582:3
    |
    |
582 | /   move |i: I| {
583 | |     let mut input = i.clone();
584 | |
585 | |     for elem in buf.iter_mut() {
...   |
601 | |     Ok((input, ()))
602 | |   }
    | |___^ ...so that the type `I` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
    |
578 |   I: Clone + PartialEq + 'a,

   Compiling rand_chacha v0.3.0
   Compiling itertools v0.10.1
   Compiling minifier v0.2.2
