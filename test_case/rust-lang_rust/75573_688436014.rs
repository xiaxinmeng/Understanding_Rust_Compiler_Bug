diff
-error: assignment to temporary
-  --> $DIR/temporary_assignment.rs:47:5
-   |
-LL |     Struct { field: 0 }.field = 1;
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::temporary-assignment` implied by `-D warnings`
-
-error: assignment to temporary
-  --> $DIR/temporary_assignment.rs:48:5
-   |
-LL | /     MultiStruct {
-LL | |         structure: Struct { field: 0 },
-LL | |     }
-LL | |     .structure
-LL | |     .field = 1;
-   | |______________^
-
-error: assignment to temporary
-  --> $DIR/temporary_assignment.rs:53:5
-   |
-LL |     ArrayStruct { array: [0] }.array[0] = 1;
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: assignment to temporary
-  --> $DIR/temporary_assignment.rs:54:5
-   |
-LL |     (0, 0).0 = 1;
-   |     ^^^^^^^^^^^^
-
-error: assignment to temporary
+error: attempting to modify a `const` item
   --> $DIR/temporary_assignment.rs:56:5
    |
 LL |     A.0 = 2;
    |     ^^^^^^^
+   |
+   = note: `-D const-item-mutation` implied by `-D warnings`
+   = note: each usage of a `const` item creates a new temporary - the original `const` item will not be modified
+note: `const` item defined here
+  --> $DIR/temporary_assignment.rs:36:1
+   |
+LL | const A: TupleStruct = TupleStruct(1);
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

-error: assignment to temporary
+error: attempting to modify a `const` item
   --> $DIR/temporary_assignment.rs:57:5
    |
 LL |     B.field = 2;
    |     ^^^^^^^^^^^
+   |
+   = note: each usage of a `const` item creates a new temporary - the original `const` item will not be modified
+note: `const` item defined here
+  --> $DIR/temporary_assignment.rs:37:1
+   |
+LL | const B: Struct = Struct { field: 1 };
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

-error: assignment to temporary
+error: attempting to modify a `const` item
   --> $DIR/temporary_assignment.rs:58:5
    |
 LL |     C.structure.field = 2;
    |     ^^^^^^^^^^^^^^^^^^^^^
+   |
+   = note: each usage of a `const` item creates a new temporary - the original `const` item will not be modified
+note: `const` item defined here
+  --> $DIR/temporary_assignment.rs:38:1
+   |
+LL | / const C: MultiStruct = MultiStruct {
+LL | |     structure: Struct { field: 1 },
+LL | | };
+   | |__^

-error: assignment to temporary
+error: attempting to modify a `const` item
   --> $DIR/temporary_assignment.rs:59:5
    |
 LL |     D.array[0] = 2;
    |     ^^^^^^^^^^^^^^
+   |
+   = note: each usage of a `const` item creates a new temporary - the original `const` item will not be modified
+note: `const` item defined here
+  --> $DIR/temporary_assignment.rs:41:1
+   |
+LL | const D: ArrayStruct = ArrayStruct { array: [1] };
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

-error: aborting due to 8 previous errors
+error: aborting due to 4 previous errors
