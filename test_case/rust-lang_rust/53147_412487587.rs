plain
[00:49:24] ....................................................................................................
[00:49:27] ....................................................................................................
[00:49:30] ....................................................................................................
[00:49:34] ......................i.............................................................................
[00:49:37] ......F..........................i..................................................................
[00:49:44] ....................................................................................................
[00:49:48] ........................................................i...........................................
 does not implement the `Copy` trait
 does not implement the `Copy` trait
[00:49:50] +    |           |    help: consider removing the `*`: `a`
[00:49:50] +    |           data moved here
[00:49:50] +    |
[00:49:50] + note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[00:49:50] +   --> $DIR/move-errors.rs:51:11
[00:49:50] +    |
[00:49:50] + LL |     let A(s) = *a;
[00:49:50] 55 
[00:49:50] 55 
[00:49:50] 56 error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[00:49:50] 
[00:49:50] 60    |             -     ^ cannot move out of here
[00:49:50] 61    |             |
[00:49:50] 62    |             data moved here
[00:49:50] 62    |             data moved here
[00:49:50] -    |             help: to prevent move, use ref or ref mut: `ref s`
[00:49:50] +    |
[00:49:50] + note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[00:49:50] +   --> $DIR/move-errors.rs:57:13
[00:49:50] +    |
[00:49:50] + LL |     let C(D(s)) = c;
[00:49:50] 64 
[00:49:50] 65 error[E0507]: cannot move out of borrowed content
[00:49:50] 66   --> $DIR/move-errors.rs:64:9
[00:49:50] 
[00:49:50] 
[00:49:50] 75    |           ^^^^
[00:49:50] 76    |           |
[00:49:50] 77    |           cannot move out of here
[00:49:50] -    |           help: consider using a reference instead: `&x[0]`
[00:49:50] +    |           help: consider borrowing here: `&x[0]`
[00:49:50] 79 LL |     //~^ ERROR
[00:49:50] 80 LL |         B::U(d) => (),
[00:49:50] -    |              - move occurs because d has type `D`, wh:49:50] 120 
[00:49:50] 121 error[E0509]: cannot move out of type `F`, which implements the `Drop` trait
[00:49:50] 
[00:49:50] 128    |           -  ----- ... and here
[00:49:50] 129    |           |
[00:49:50] 130    |           data moved here
[00:49:50] 130    |           data moved here
[00:49:50] - help: to prevent move, use ref or ref mut
[00:49:50] 132    |
[00:49:50] - LL |         F(ref s, ref mut t) => (),
[00:49:50] -    |           ^^^^^  ^^^^^^^^^
[00:49:50] + note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[00:49:50] +   --> $DIR/move-errors.rs:117:11
[00:49:50] +    |
[00:49:50] + LL |         F(s, mut t) => (),
[00:49:50] +    |           ^
[00:49:50] + note: move occurs because `t` has type `std::string::String`, which does not implement the `Copy` trait
[00:49:50] +   --> $DIR/move-errors.rs:117:14
[00:49:50] +    |
[00:49:50] + LL |         F(s, mut t) => (),
[00:49:50] 135 
[00:49:50] 136 error[E0507]: cannot move out of borrowed content
[00:49:50] 137   --> $DIR/move-errors.rs:123:11
[00:49:50] 
[00:49:50] 
[00:49:50] 140    |           ^^
[00:49:50] 141    |           |
[00:49:50] 142    |           cannot move out of borrowed content
[00:49:50] -    |           help: consider removing this dereference operator: `x`
[00:49:50] +    |           help: consider removing the `*`: `x`
[00:49:50] 144 LL |     //~^ ERROR
[00:49:50] 145 LL |         Ok(s) | Err(s) => (),
[00:49:50] -    |            - move occurs because s has type `std::string::String`, which does not implement the `Copy` trait
[] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n