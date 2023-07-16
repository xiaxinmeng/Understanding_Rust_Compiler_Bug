
37 | let mut iter = err.iter_chain();
   |                    ^^^^^^^^^^ multiple `iter_chain` found
   |
   = note: candidate #1 is defined in an impl for the type `(dyn Error + 'static)`
note: candidate #2 is defined in the trait `Error`
