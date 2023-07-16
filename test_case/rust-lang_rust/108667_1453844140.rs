
---- [ui] tests/ui/typeck/bad-type-in-vec-contains.rs stdout ----
diff of stderr:

4       LL |     primes.contains(3);
5          |            -------- ^
6          |            |        |
-          |            |        expected `&_`, found integer
+          |            |        expected reference, found integer
8          |            |        help: consider borrowing here: `&3`
9          |            arguments to this method are incorrect
10         |            here the type of `primes` is inferred to be `[_]`

11         |
12         = note: expected reference `&_`
13                         found type `{integer}`
-       note: method defined here
+       note: associated function defined here
15        --> $SRC_DIR/core/src/slice/mod.rs:LL:COL
16
17      error: aborting due to previous error
