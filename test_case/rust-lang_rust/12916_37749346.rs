 rust
macro_rules! test {
     ($($($e: expr),*);*, $x: ident) => {
         (#($e),
          #($($e)*), 
          #($($($e)*)*),
          #($($e $e)*),
          #($($x $e)*),
          #($x),
          #($x $x),
          #($e $x),
          #(1))
          // etc.
     }
}
