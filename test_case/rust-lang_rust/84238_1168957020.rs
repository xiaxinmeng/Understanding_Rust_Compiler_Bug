rust
#![feature(adt_const_params)]

struct B<const F: [fn(); 1]>;
