rust
macro_rules! foo {
    ($d:tt) => {
        macro_rules! bar {
            // The lint does not recognize the possible meta-variable occurrence:
            // `$d z` is a free meta-variable if $d is `$`.
            ($y:tt) => { $d z };
        }
    };
}
