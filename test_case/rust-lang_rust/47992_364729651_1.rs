rust
struct T; // (a)

#[defines(S, T)]
macro m() {
    struct S; // (b) defines `S` at the call site *and* the def site
    S // resolves at the def site, so it always resolves to (b)
    T // `T` resolves the def site (hygienically), so it always resolves to (a)
}
