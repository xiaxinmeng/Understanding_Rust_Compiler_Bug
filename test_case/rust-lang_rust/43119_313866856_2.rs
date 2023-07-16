rust
macro_rules! remainder  {
    ($dividend:expr, $divisor:expr) => {
        #[allow(const_err)]
        match $divisor == 0 {
            true => 0,
            false => $dividend % $divisor
        }
    };
}

fn main()   {
    #[allow(const_err)]
    let _ = remainder!(12, 0);
}
