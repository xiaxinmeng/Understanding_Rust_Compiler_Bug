
use std::io;

struct T (&'static [int]);
static t : T = T (&'static [5, 4, 3]);
fn main () {
    assert_eq!(t[0], 5);
}
