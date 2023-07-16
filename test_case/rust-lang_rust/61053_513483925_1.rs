rust
macro_rules! foo { ($($x:tt)*) => {}; }
macro_rules! bar { () => { foo!($x) }; }
