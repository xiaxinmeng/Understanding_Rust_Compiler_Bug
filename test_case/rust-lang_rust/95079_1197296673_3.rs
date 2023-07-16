
error[E0499]: cannot borrow `x` as mutable more than once at a time
 --> /home/ychen/hello_world/src/main_closue_body.rs:7:13
  |
6 |             let y = &mut x;
  |                     ------ first mutable borrow occurs here
7 |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
  |             ^^^^^^ second mutable borrow occurs here
8 |             *y = 1;
  |             ------ first borrow later used here

error[E0499]: cannot borrow `x` as mutable more than once at a time
  --> /home/ychen/hello_world/src/main_closue_body.rs:17:20
   |
16 |                    let y = &mut x;
   |                            ------ first mutable borrow occurs here
17 |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |                    ^^^^^^ second mutable borrow occurs here
18 |                    *y = 1;
   |                    ------ first borrow later used here

error: captured variable cannot escape `FnMut` closure body
  --> /home/ychen/hello_world/src/main_closue_body.rs:15:16
   |
13 |           let mut x = 0;
   |               ----- variable defined here
14 |              || {
   |               - inferred to be a `FnMut` closure
15 | /                || { //~ ERROR captured variable cannot escape `FnMut` closure body
16 | |                    let y = &mut x;
   | |                                 - variable captured here
17 | |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
18 | |                    *y = 1;
19 | |                    drop(y);
20 | |                 }
   | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
help: consider adding 'move' keyword before the nested closure
   |
15 |                move || { //~ ERROR captured variable cannot escape `FnMut` closure body
   |                ++++

error: lifetime may not live long enough
  --> /home/ychen/hello_world/src/main_closue_body.rs:26:13
   |
25 |         let _action = move || {
   |                       -------
   |                       |     |
   |                       |     return type of closure `[closure@/home/ychen/hello_world/src/main_closue_body.rs:26:13: 26:15]` contains a lifetime `'2`
   |                       lifetime `'1` represents this closure's body
26 |             || f() // The `nested` closure
   |             ^^^^^^ returning this value requires that `'1` must outlive `'2`
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
help: consider adding 'move' keyword before the nested closure
   |
26 |             move || f() // The `nested` closure
   |             ++++

error: aborting due to 4 previous errors
