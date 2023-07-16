diff
---- [ui] ui/issue-23302-2.rs stdout ----
        diff of stderr:

-       error[E0391]: cycle detected when processing `Y::A::{{initializer}}`
+       error[E0391]: cycle detected when const-evaluating `Y::A::{{initializer}}`
2         --> $DIR/issue-23302-2.rs:14:9
3          |
4       LL |     A = Y::B as isize, //~ ERROR E0391

-          |         ^^^^^^^^^^^^^
+          |         ^^^^
6          |
-          = note: ...which again requires processing `Y::A::{{initializer}}`, completing the cycle
-       note: cycle used when const-evaluating `Y::A::{{initializer}}`
-         --> $DIR/issue-23302-2.rs:14:9
-          |
-       LL |     A = Y::B as isize, //~ ERROR E0391
-          |         ^^^^^^^^^^^^^
+       note: ...which requires computing layout of `Y`...
+          = note: ...which again requires const-evaluating `Y::A::{{initializer}}`, completing the cycle
