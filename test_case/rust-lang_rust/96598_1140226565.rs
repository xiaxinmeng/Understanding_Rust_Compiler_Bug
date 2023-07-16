rust
struct A;
struct B;
struct C;

fn whoops(_: A, _: B, _: A) { }

fn main() {
    whoops(B, C);
}
