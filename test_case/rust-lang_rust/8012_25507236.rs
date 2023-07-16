 .rs
macro_rules! omg(($n:expr) => ($n,$n+1))
omg!(100) // => 100, not (100, 101)
