
$ RUST_BACKTRACE=full rustc +stage1 src/test/run-pass/dynamic-drop.rs -Znll -Zborrowck=mir -Ztwo-phase-borrows
warning: variable `x` is assigned to, but never used
   --> src/test/run-pass/dynamic-drop.rs:187:10
    |
187 |     let (x, y, z);
    |          ^
    |
    = note: #[warn(unused_variables)] on by default
    = note: to avoid this warning, consider using `_x` instead

warning: variable `y` is assigned to, but never used
   --> src/test/run-pass/dynamic-drop.rs:187:13
    |
187 |     let (x, y, z);
    |             ^
    |
    = note: to avoid this warning, consider using `_y` instead

warning: variable `z` is assigned to, but never used
   --> src/test/run-pass/dynamic-drop.rs:187:16
    |
187 |     let (x, y, z);
    |                ^
    |
    = note: to avoid this warning, consider using `_z` instead

warning: value assigned to `x` is never read
   --> src/test/run-pass/dynamic-drop.rs:188:5
    |
188 |     x = a.alloc();
    |     ^
    |
    = note: #[warn(unused_assignments)] on by default

warning: value assigned to `y` is never read
   --> src/test/run-pass/dynamic-drop.rs:189:5
    |
189 |     y = 5;
    |     ^

warning: value assigned to `z` is never read
   --> src/test/run-pass/dynamic-drop.rs:190:5
    |
190 |     z = a.alloc();
    |     ^

[santiago@archlinux rust1 (master)]$ 
