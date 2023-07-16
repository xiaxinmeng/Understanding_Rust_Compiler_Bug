rs
// due to compiler wrongfully complaining re: Copy impl missing for packed struct
#![allow(unaligned_references)]
#[macro_use]
extern crate hdf5_derive;
#[derive(H5Type, Copy, Clone)]
#[repr(packed)]
struct P2(i8, u32);
