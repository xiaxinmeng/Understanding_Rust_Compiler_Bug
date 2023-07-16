rust
struct Error;
type Result<T, E = Error> = core::result::Result<T, E>;

fn foo<T, E, F>(f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    f()
}

fn main() {
    // This says: infer both `T` and `E` for the `Result`
    // Since `E` is unknown, this fails to compile.
    // let _ = foo(|| Result::Ok(10));

    // This says: infer `T`, but use the default `E` of `Error`.
    let _ = foo(|| Result::<_>::Ok(10));
}
