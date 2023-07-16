
   Compiling nalgebra v0.10.1
error[E0391]: unsupported cyclic reference between types/traits detected
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\nalgebra-0.10.1\src\structs\dmatrix.rs:148:5
    |
148 |     mij:   [N; 2 * 2],
    |     ^^^^^^^^^^^^^^^^^ cyclic reference
    |
note: the cycle begins when processing `structs::dmatrix::DMatrix2::mij`...
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\nalgebra-0.10.1\src\structs\dmatrix.rs:148:5
    |
148 |     mij:   [N; 2 * 2],
    |     ^^^^^^^^^^^^^^^^^
note: ...which then requires const-evaluating `structs::dmatrix::DMatrix2::{{initializer}}`...
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\nalgebra-0.10.1\src\structs\dmatrix.rs:148:16
    |
148 |     mij:   [N; 2 * 2],
    |                ^^^^^
note: ...which then requires processing `structs::dmatrix::DMatrix2::{{initializer}}`...
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\nalgebra-0.10.1\src\structs\dmatrix.rs:148:16
    |
148 |     mij:   [N; 2 * 2],
    |                ^^^^^
note: ...which then requires coherence checking all impls of trait `std::ops::Mul`...
note: ...which then requires processing `structs::dmatrix::DMatrix`...
   --> C:\Users\John\.cargo\registry\src\github.com-1ecc6299db9ec823\nalgebra-0.10.1\src\structs\dmatrix.rs:20:1
    |
20  | / pub struct DMatrix<N> {
21  | |     nrows: usize,
22  | |     ncols: usize,
23  | |     mij:   Vec<N>
24  | | }
    | |_^
note: ...which then requires computing the variances for items in this crate...
    = note: ...which then again requires processing `structs::dmatrix::DMatrix2::mij`, completing the cycle.
