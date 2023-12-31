diff
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:65:5
+error: taking a mutable reference to a `const` item
+  --> $DIR/borrow_interior_mutable_const.rs:72:21
    |
-LL |     ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability
-   |     ^^^^^^
-   |
-   = note: `-D clippy::borrow-interior-mutable-const` implied by `-D warnings`
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:66:16
-   |
-LL |     assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability
-   |                ^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:69:22
-   |
-LL |     let _once_ref = &ONCE_INIT; //~ ERROR interior mutability
-   |                      ^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:70:25
-   |
-LL |     let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability
-   |                         ^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:71:27
-   |
-LL |     let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability
-   |                           ^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:72:26
-   |
 LL |     let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability
-   |                          ^^^^^^^^^
+   |                     ^^^^^^^^^^^^^^
    |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:83:14
+   = note: `-D const-item-mutation` implied by `-D warnings`
+   = note: each usage of a `const` item creates a new temporary
+   = note: the mutable reference will refer to this temporary, not the original `const` item
+note: `const` item defined here
+  --> $DIR/borrow_interior_mutable_const.rs:19:1
    |
-LL |     let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability
-   |              ^^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
+LL | const ONCE_INIT: Once = Once::new();
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:84:14
-   |
-LL |     let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability
-   |              ^^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:85:19
-   |
-LL |     let _ = &(&&&&ATOMIC_TUPLE).0; //~ ERROR interior mutability
-   |                   ^^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:86:14
-   |
-LL |     let _ = &ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
-   |              ^^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:87:13
-   |
-LL |     let _ = ATOMIC_TUPLE.0[0].load(Ordering::SeqCst); //~ ERROR interior mutability
-   |             ^^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:93:13
-   |
-LL |     let _ = ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
-   |             ^^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:98:5
-   |
-LL |     CELL.set(2); //~ ERROR interior mutability
-   |     ^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:99:16
-   |
-LL |     assert_eq!(CELL.get(), 6); //~ ERROR interior mutability
-   |                ^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:112:5
-   |
-LL |     u64::ATOMIC.store(5, Ordering::SeqCst); //~ ERROR interior mutability
-   |     ^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: a `const` item with interior mutability should not be borrowed
-  --> $DIR/borrow_interior_mutable_const.rs:113:16
-   |
-LL |     assert_eq!(u64::ATOMIC.load(Ordering::SeqCst), 9); //~ ERROR interior mutability
-   |                ^^^^^^^^^^^
-   |
-   = help: assign this const to a local or static variable, and use the variable here
-
-error: aborting due to 16 previous errors
+error: aborting due to previous error
