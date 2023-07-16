
   |
8 |         println!("{}", y); //~ ERROR use of moved value: `y`
   |                       ^ value used here after move
9 |         while true { while true { while true { x = y; x.clone(); } } }
   |                                                   - value moved here in previous iteration of loop
   |
