rust

error[E0308]: mismatched types
  --> $DIR/closure-arg-count.rs:3:24
   |
 3 |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
   |                        ^^^^^^^^^^^^^^^ expected &{integer}, found tuple
   |
   = note: expected type `&{integer}`
              found type `(_, _)`

error[E0593]: closure takes 1 argument but 2 arguments are required
  --> $DIR/closure-arg-count.rs:3:15
   |
 3 |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
   |               ^^^^^^^ -------------------------- takes 1 argument
   |               |
   |               expected closure that takes 2 arguments


error: aborting due to 2 previous errors
