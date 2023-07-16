
error[E0267]: `break` inside of an `async` closure
  --> $DIR/async-block-control-flow-static-semantics.rs:35:9
   |
LL |       async {
   |  ___________-
LL | |         break 0u8;
   | |         ^^^^^^^^^ cannot `break` inside of an `async` closure
LL | |     };
   | |_____- enclosing `async` closure
