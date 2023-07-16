rust
error[E0593]: closure takes 1 parameter but 2 parameters are required here
  --> $DIR/closure-arg-count.rs:12:15
   |
12 |     [1, 2, 3].sort_by(|tuple| panic!());
   |               ^^^^^^^ ---------------- takes 1 parameter
   |               |
   |               expected closure that takes 2 parameters

error[E0593]: closure takes 1 parameter but 2 parameters are required here
  --> $DIR/closure-arg-count.rs:12:15
   |
12 |     [1, 2, 3].sort_by(|tuple| panic!());
   |               ^^^^^^^ ---------------- takes 1 parameter
   |               |
   |               expected closure that takes 2 parameters

error[E0308]: mismatched types
  --> $DIR/closure-arg-count.rs:13:24
   |
13 |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
   |                        ^^^^^^^^^^^^^^^ expected &{integer}, found tuple
   |
   = note: expected type `&{integer}`
              found type `(_, _)`

error[E0593]: closure takes 1 parameter but 2 parameters are required here
  --> $DIR/closure-arg-count.rs:13:15
   |
13 |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
   |               ^^^^^^^ -------------------------- takes 1 parameter
   |               |
   |               expected closure that takes 2 parameters

error[E0593]: closure takes 1 parameter but 2 parameters are required here
  --> $DIR/closure-arg-count.rs:13:15
   |
13 |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
   |               ^^^^^^^ -------------------------- takes 1 parameter
   |               |
   |               expected closure that takes 2 parameters

error: aborting due to 5 previous errors
