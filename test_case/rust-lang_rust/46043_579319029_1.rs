
error[E0308]: mismatched types
  --> prog.rs:10:19
   |
10 |     let y: &u32 = &x.q;
   |                   ^^^^ types differ in alignment guarantees
   |
   = note: expected type `&u32`
              found type `&unalign u32`
