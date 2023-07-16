rust
#![feature(catch_expr)]

fn transpose<T, E1, E2>(x: Result<Result<T, E1>, E2>) -> Result<Result<T, E2>, E1> {
    do catch {
        Ok(x??)
    }
}
