 rust
macro_rules! do_if_true{
    ($e:expr { $($b:tt)* }) => {
        if $e { $($b)* }
        else {
            println!("got false");
        }
    };
}
