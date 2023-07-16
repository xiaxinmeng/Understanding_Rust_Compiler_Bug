
error[E0276]: impl has stricter requirements than trait
  --> test.rs:13:5
   |
5  | /     fn process_backedge<'a, I>(&mut self)
6  | |         where I: Iterator<Item=&'a Self::Obligation>,
7  | |               Self::Obligation: 'a;
   | |___________________________________- definition of `process_backedge` from trait
...
13 | /     fn process_backedge<'c, I>(&mut self)
14 | |         where I: Iterator<Item=&'c &'tcx ()>,
15 | |         'tcx: 'c
16 | |     {
17 | |         loop {}
18 | |     }
   | |_____^ impl has extra requirement `'tcx: 'c`

error: aborting due to previous error
