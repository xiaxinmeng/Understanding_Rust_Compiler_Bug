 rust
#[feature(macro_rules)];

macro_rules! item { ($i: item) => { $i } }

macro_rules! foo (
    ($x:tt) => { item!(type t = $x int;) }
)

foo!(~)

fn main() {}
