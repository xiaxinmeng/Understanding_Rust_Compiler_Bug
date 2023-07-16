rust
macro_rules! eq {
    ($a:tt $b:tt) => {
        macro_rules! __eq {
            ($a $a) => { /* equal */ };
            ($a $b) => { /* not equal */ }
        }
        
        __eq!($a $b);
    }
}
