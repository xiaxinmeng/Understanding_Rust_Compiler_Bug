
warning: irrefutable while-let pattern
  --> $DIR/while-let.rs:27:5
   |
LL | /     while let _a = 1 {
LL | |         println!("irrefutable pattern");
LL | |         break;
LL | |     }
   | |_____^
   |
   = note: this pattern will always match, so the loop will never exit
   = help: consider using a `loop { ... }` and adding a `let` inside it
