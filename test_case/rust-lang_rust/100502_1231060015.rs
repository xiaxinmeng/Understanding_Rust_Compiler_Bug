rust
struct A;
struct B;
struct C;
struct D;

fn f(
    a1: A,
    a2: A,
    b1: B,
    b2: B,
    c1: C,
    c2: C,
) {}

fn main() {
    f(C, C, A, A, B, B);
}
