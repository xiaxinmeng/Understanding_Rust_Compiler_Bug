rust
use std::fmt::{Debug, Formatter, Result};

struct Arm<'a, L: 'a, R: 'a>(&'a (L, R));
struct Table<'a, K: 'a, V: 'a>(&'a [(K, V)], V);

impl<'a, L: 'a + Debug, R: 'a + Debug> Debug for Arm<'a, L, R> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        L::fmt(&(self.0).0, fmt)?;
        fmt.write_str(" => ")?;
        R::fmt(&(self.0).1, fmt)
    }
}

impl<'a, K: 'a + Debug, V: 'a + Debug> Debug for Table<'a, K, V> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.debug_set()
           .entries(self.0.iter().map(Arm))
           .entry(&Arm(&(format_args!("{}", "_"), &self.1)))
           .finish()
    }
}

fn main() {
    let table = (1..3).enumerate().collect::<Vec<_>>();
    assert_eq!(format!("{:?}", Table(&*table, 0)),
               "{0 => 1, 1 => 2, _ => 0}");
}
