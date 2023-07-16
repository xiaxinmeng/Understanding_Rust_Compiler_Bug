text
error[E0282]: type annotations needed
  --> src/main.rs:8:1
   |
8  | / fn foo<'t,X,T:'t>()
9  | |     where X: for<'a> ImmutableSequenceTypes<'a, T>,
10 | |           X: ImmutableSequenceTypes<'t, T, Output=&'t T>
11 | | {
12 | |     bar::<<X as ImmutableSequenceTypes<'t,T>>::Output>();
13 | | }
   | |_^ cannot infer type
