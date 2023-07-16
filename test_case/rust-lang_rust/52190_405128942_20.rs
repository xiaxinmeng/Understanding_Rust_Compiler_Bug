\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-45697.rs","byte_start":936,"byte_end":937,"line_start":29,"line_end":29,"column_start":40,"column_end":41,"is_primary":false,"text":[{"text":"        let z = copy_borrowed_ptr(&mut y);","highlight_start":40,"highlight_end":41}],"label":"borrow of `*y.pointer` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-45697.rs","byte_start":948,"byte_end":963,"line_start":30,"line_end":30,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        *y.pointer += 1;","highlight_start":9,"highlight_end":24}],"label":"assignment to borrowed `*y.pointer` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0506]: cannot assign to `*y.pointer` because it is borrowed (Ast)\n  --> /checkout/src/test/ui/issue-45697.rs:30:9\n   |\nLL |         let z = copy_borrowed_ptr(&mut y);\n   |                                        - borrow of `*y.pointer` occurs here\nLL |         *y.pointer += 1;\n   |         ^^^^^^^^^^^^^^^ assignment to borrowed `*y.pointer` occurs here\n\n"}
[00:42:59] thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2', /checkout/src/libcore/slice/mod.rs:2085:10
[00:42:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:59] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:59] {"message":"For more information about this error, try `rustc --explain E0506`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0506`.\n"}
[00:42:59] error: internal compiler error: unexpected panic
---
[00:42:59] 18 
[00:42:59] - error[E0507]: cannot move out of borrowed content
[00:42:59] -   --> $DIR/move-errors.rs:32:13
[00:42:59] -    |
[00:42:59] - LL |     let s = **r;
[00:42:59] -    |             |
[00:42:59] -    |             cannot move out of borrowed content
[00:42:59] -    |             help: consider using a reference instead: `&**r`
[00:42:59] + error: aborting due to 2 previous errors
[00:42:59] + error: aborting due to 2 previous errors
[00:42:59] 27 
[00:42:59] - error[E0507]: cannot move out of borrowed content
[00:42:59] -   --> $DIR/move-errors.rs:40:13
[00:42:59] -    |
[00:42:59] - LL |     let s = *r;
[00:42:59] -    |             |
[00:42:59] -    |             cannot move out of borrowed content
[00:42:59] -    |             help: consider using a reference instead: `&*r`
[00:42:59] - 
[00:42:59] - 
[00:42:59] - error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     let a = [A("".to_string())][0];
[00:42:59] -    |             |
[00:42:59] -    |             cannot move out of here
[00:42:59] -    |             cannot move out of here
[00:42:59] -    |             help: consider using a reference instead: `&[A("".to_string())][0]`
[00:42:59] - error[E0507]: cannot move out of borrowed content
[00:42:59] -   --> $DIR/move-errors.rs:51:16
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     let A(s) = *a;
[00:42:59] -    |           -    ^^
[00:42:59] -    |           |    cannot move out of borrowed content
[00:42:59] -    |           |    help: consider removing this dereference operator: `a`
[00:42:59] -    |           |    help: consider removing this dereference operator: `a`
[00:42:59] -    |           move occurs because s has type `std::string::String`, which does not implement the `Copy` trait
[00:42:59] - 
[00:42:59] - error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     let C(D(s)) = c;
[00:42:59] -    |             -     ^ cannot move out of here
[00:42:59] -    |             |
[00:42:59] -    |             help: to prevent move, use ref or ref mut: `ref s`
[00:42:59] - error[E0507]: cannot move out of borrowed content
[00:42:59] -   --> $DIR/move-errors.rs:64:9
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     b = *a;
[00:42:59] -    |         ^^ cannot move out of borrowed content
[00:42:59] - 
[00:42:59] - error[E0508]: cannot move out of type `[B; 1]`, a non-copy array
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     match x[0] {
[00:42:59] -    |           |
[00:42:59] -    |           cannot move out of here
[00:42:59] -    |           help: consider using a reference instead: `&x[0]`
[00:42:59] -    |           help: consider using a reference instead: `&x[0]`
[00:42:59] - LL |     //~^ ERROR
[00:42:59] - LL |         B::U(d) => (),
[00:42:59] -    |              - move occurs because d has type `D`, which does not implement the `Copy` trait
[00:42:59] - LL |         B::V(s) => (),
[00:42:59] -    |              - move occurs because s has type `std::string::String`, which does not implement the `Copy` trait
[00:42:59] - 
[00:42:59] - error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     match x {
[00:42:59] -    |           ^ cannot move out of here
[00:42:59] - ...
[00:42:59] - LL |         B::U(D(s)) => (),
[00:42:59] -    |                - help: to prevent move, use ref or ref mut: `ref s`
[00:42:59] - 
[00:42:59] - error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     match x {
[00:42:59] -    |           ^ cannot move out of here
[00:42:59] - ...
[00:42:59] - LL |         (D(s), &t) => (),
[00:42:59] -    |            - help: to prevent move, use ref or ref mut: `ref s`
[00:42:59] - error[E0507]: cannot move out of borrowed content
[00:42:59] -   --> $DIR/move-errors.rs:105:11
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     match x {
[00:42:59] -    |           ^ cannot move out of borrowed content
[00:42:59] - ...
[00:42:59] - LL |         (D(s), &t) => (),
[00:42:59] -    |                 - help: to prevent move, use ref or ref mut: `ref t`
[00:42:59] - 
[00:42:59] - error[E0509]: cannot move out of type `F`, which implements the `Drop` trait
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     match x {
[00:42:59] -    |           ^ cannot move out of here
[00:42:59] - help: to prevent eDarkKnight;\n\nimpl TheDarkKnight {\n    fn nothing_is_true(&self) {} // First case, we don't take ownership\n}\n\nfn main() {\n    let x = RefCell::new(TheDarkKnight);\n\n    x.borrow().nothing_is_true(); // ok!\n}\n