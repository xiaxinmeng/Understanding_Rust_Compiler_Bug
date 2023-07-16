
error[E0282]: type annotations needed
  --> input.rs:10:5
   |
10 |     <() as Trait<'static, _>>::m();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `'a` declared on the trait `Trait`

error: aborting due to previous error
