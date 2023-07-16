
struct S;

fn main() {
    let S = S; // error: declaration of `S` shadows an enum variant or unit-like struct in scope
}
