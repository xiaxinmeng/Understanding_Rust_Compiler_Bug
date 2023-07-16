
error[E0308]: mismatched types
  --> src/main.rs:18:15
   |
13 | /     move || {
14 | |         let _ = capture;
15 | |     }
   | |_____- the found closure
...
18 |   let f: fn() = closure;
   |          ----   ^^^^^^^ expected fn pointer, found closure
   |          |
   |          expected due to this
   |
   = note: expected fn pointer `fn()`
                 found closure `[closure@src/main.rs:13:5: 15:6]`
note: closures can only be coerced to `fn` types if they do not capture any variables
  --> src/main.rs:14:17
   |
14 |         let _ = capture;
   |                 ^^^^^^^ `capture` captured here
