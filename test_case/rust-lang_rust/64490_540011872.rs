
trait AddTwo {
    fn add_two(&mut self);
}

impl AddTwo for u32 {
    fn add_two(&mut self) { *self += 2; }
}

const N: u32 = 0;
// somewhere else
assert_eq!(N, 0);
// N += 2; // doesn't compile
N.add_two(); // compiles with no warnings
assert_eq!(N, 2); // fails
