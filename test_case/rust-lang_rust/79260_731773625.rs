rust
struct S;

#[transparent]
mod m {
    fn f() {
        let s = S; // OK
    }
}
