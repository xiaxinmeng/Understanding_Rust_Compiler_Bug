rust
#![feature(stmt_expr_attributes)]

macro_rules! remainder  {
    ($dividend:expr, $divisor:expr) => {
        if $divisor == 0 {
            0
        } else {
            #[allow(const_err)]
            $dividend % $divisor
        }
    };
}

fn main()   {
    #[allow(const_err)]
    let _ = remainder!(12, 0);
}
