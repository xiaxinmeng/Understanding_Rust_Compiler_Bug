
error[E0053]: method `index` has an incompatible type for trait
  --> test.rs:8:5
   |
8  | /     fn index<'a>(&'a self, index: u16) -> u8 {
9  | |         0
10 | |     }
   | |_____^ expected &u8, found u8
   |
   = note: expected type `fn(&Interconnect, u16) -> &u8`
              found type `fn(&Interconnect, u16) -> u8`

expected type `fn(&Interconnect, u16) -> &u8`, derived from 

2   |impl Index<u16> for Interconnect {
3.  |    type Output = u8;
                    ^------- Output is u8, so signature expects to return &u8

error: aborting due to previous error(s)
