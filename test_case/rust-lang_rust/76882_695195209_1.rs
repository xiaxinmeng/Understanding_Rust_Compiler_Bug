 
error[E0310]: the parameter type `I` may not live long enough
 --> src/lib.rs:4:29
  |
4 | fn erase<I: Trait>(x: I) -> impl Trait + 'static {
  |          --                 ^^^^^^^^^^^^^^^^^^^^ ...so that the type `I` will meet its required lifetime bounds
  |          |
  |          help: consider adding an explicit lifetime bound...: `I: 'static +`
