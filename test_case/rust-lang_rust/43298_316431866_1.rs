

error[E0623]: lifetime mismatch
  --> src/test/ui/lifetime-errors/ex3-both-anon-regions-2.rs:12:13
   |
11 | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
   |                     ---                 --- these references must have the same lifetime
12 |     z.push((x,y));
   |             ^ data  flows into `z` here

error[E0623]: lifetime mismatch
  --> src/test/ui/lifetime-errors/ex3-both-anon-regions-2.rs:12:15
   |
11 | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
   |                         ---                  --- these references must have the same lifetime
12 |     z.push((x,y));
   |               ^ data  flows into `z` here

error: aborting due to 2 previous errors
