Rust
#![feature(adt_const_params)]

struct Works<const F: [*mut (); 1]>; // Should not work

struct DoesNotWork<const F: *mut ()>;
