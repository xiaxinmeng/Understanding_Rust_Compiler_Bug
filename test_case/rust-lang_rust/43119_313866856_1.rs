rust
macro_rules! remainder  {
    ($dividend:expr, $divisor:expr) => {
        #[allow(const_err)]
        if $divisor == 0 {
            0
        } else {
            $dividend % $divisor
        }
    };
}

fn main()   {
    #[allow(const_err)]
    let _ = remainder!(12, 0);
}
