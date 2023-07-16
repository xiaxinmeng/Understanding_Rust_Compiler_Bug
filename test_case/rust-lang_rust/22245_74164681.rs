 rust
macro_rules! foo {
     ($x: ident) => {
         $x!("")
     }
}

foo!(asm)
