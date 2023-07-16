
fn f() {
    struct S;

    mod m {
        pub fn f() -> S { S } // Private-in-public error unless `struct S` is public
    }
}

fn main() {
}
