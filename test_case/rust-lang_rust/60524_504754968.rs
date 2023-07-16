rust
macro_rules! m {
    ($e:expr) => { "expr" };
    ($($t:tt)*) => { "tts" };
}

// dbg!(m!(if)); // <error>
dbg!(m!(if true {})); // expr
dbg!(m!(if true {} whatever)); // tts
// dbg!(m!(if true {} else)); // <error>
dbg!(m!(if true {} else {} ;)); // tts
