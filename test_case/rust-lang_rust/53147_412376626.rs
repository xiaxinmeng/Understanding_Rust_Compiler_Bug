plain
[00:48:56] ....................................................................................................
[00:48:59] ....................................................................................................
[00:49:02] ....................................................................................................
[00:49:06] ....................i...............................................................................
[00:49:09] ....F..........................i....................................................................
[00:49:16] ....................................................................................................
[00:49:19] ......................................................i.............................................
[00:49:21] ..............................................i....
[00:49:21] failures:
[00:49:21] failures:
[00:49:21] 
[00:49:21] ---- [ui] ui/nll/move-errors.rs stdout ----
[00:49:21] diff of stderr:
[00:49:21] 
[00:49:21] 5    |             ^^
[00:49:21] 6    |             |
[00:49:21] 7    |             cannot move out of borrowed content
[00:49:21] -    |             help: consider removing this dereference operator: `a`
[00:49:21] +    |             help: consider removing the `*`: `a`
[00:49:21] 9 
[00:49:21] 10 error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
[00:49:21] 
[00:49:21] 14    |             ^^^^
[00:49:21] 15    |             |
[00:49:21] 16    |             cannot move out of here
[00:49:21] 16    |             cannot move out of here
[00:49:21] -    |             help: consider using a reference instead: `&a[0]`
[00:49:21] +    |             help: consider borrowing here: `&a[0]`
[00:49:21] 18 
[00:49:21] 19 error[E0507]: cannot move out of borrowed content
[00:49:21] 20   --> $DIR/move-errors.rs:32:13
[00:49:21] 
[00:49:21] 23    |             ^^^`Copy` trait
[00:49:21] +    |           |    help: consider removing the `*`: `a`
[00:49:21] +    |           data moved here
[00:49:21] +    |
[00:49:21] + note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[00:49:21] +   --> $DIR/move-errors.rs:51:11
[00:49:21] +    |
[00:49:21] + LL |     let A(s) = *a;
[00:49:21] 55 
[00:49:21] 55 
[00:49:21] 56 error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[00:49:21] 
[00:49:21] 60    |             -     ^ cannot move out of here
[00:49:21] 61    |             |
[00:49:21] 62    |             data moved here
[00:49:21] 62    |             data moved here
[00:49:21] -    |             help: to prevent move, use ref or ref mut: `ref s`
[00:49:21] +    |
[00:49:21] + note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[00:49:21] +   --> $DIR/move-errors.rs:57:13
[00:49:21] +    |
[00:49:21] + LL |     let C(D(s)) = c;
[00:49:21] 64 
[00:49:21] 65 error[E0507]: cannot move out of borrowed content
[00:49:21] 66   --> $DIR/move-errors.rs:64:9
[00:49:21] 
[00:49:21] 
[00:49:21] 75    |           ^^^^
[00:49:21] 76    |           |
[00:49:21] 77    |           cannot move out of here
[00:49:21] -    |           help: consider using a reference instead: `&x[0]`
[00:49:21] +    |           help: consider borrowing here: `&x[0]`
[00:49:21] 79 LL |     //~^ ERROR
[00:49:21] 80 LL |         B::U(d) => (),
[00:49:21] -    |              - move occurs because d has type `D`, which does not implement t} }\n# struct Batcave { knight: TheDarkKnight }\nuse std::mem;\n\nlet mut cave = Batcave {\n    knight: TheDarkKnight\n};\nlet borrowed = &mut cave;\n\nmem::replace(&mut borrowed.knight, TheDarkKnight).nothing_is_true(); // ok!\n