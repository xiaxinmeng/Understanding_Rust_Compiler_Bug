
macro_rules! m { ($($i:ident)*) => { use $($i)::*; } }
fn main() {
  m!(std cmp);
}
