rust
// Nobody would write this.
fn f<T, E, R: Into<Result<Option<T>, E>>>(input: R) {
