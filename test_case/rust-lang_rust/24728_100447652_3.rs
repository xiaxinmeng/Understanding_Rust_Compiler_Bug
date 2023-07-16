 rust
> {
>     let vs = vec![1, 2, 3, 4];
> 
>     // `for`-loops use a protocol based on the `Iterator`
>     // trait. Each item yielded in a `for` loop has the
>     // type `Iterator::Item` -- that is, `Item` is the
>     // associated type of the concrete iterator impl.
>     for v in &vs { 
> //      ~    ~~~
> //      |     |
> //      |    We borrow `vs`, iterating over a sequence of
> //      |    *references* of type `&Elem` (where `Elem` is
> //      |    vector's element type). Thus, the associated
> //      |    type `Item` must be a reference `&`-type ...
> //      |
> //  ... and `v` has the type `Iter::Item`, as dictated by
> //  the `for`-loop protocol ...
> 
>         match v {
>             1 => {}
> //          ~
> //          |
> // ... but *here*, `v` is forced to have some integral type;
> // only types like `u8`,`i8`,`u16`,`i16`, et cetera can
> // match the pattern `1` ...
> 
>             _ => {}
>         }
> 
> // ... therefore, the compiler complains, because it sees
> // an attempt to solve the equations
> // `some integral-type` = type-of-`v`
> //                      = `Item::Item`
> //                      = `&Elem` (i.e. `some reference type`)
> //
> // which cannot possibly all be true.
> 
>     }
> }
> 