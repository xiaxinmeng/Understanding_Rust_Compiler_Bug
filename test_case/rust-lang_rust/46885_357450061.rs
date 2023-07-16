
---- [ui] ui/suggestions/str-array-assignment.rs stdout ----
        diff of stderr:

2         --> $DIR/str-array-assignment.rs:13:11
3          |
4       13 |   let t = if true { s[..2] } else { s };
+          |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected str, found &str
-          |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected str, found &str
6          |
7          = note: expected type `str`
8                     found type `&str`
