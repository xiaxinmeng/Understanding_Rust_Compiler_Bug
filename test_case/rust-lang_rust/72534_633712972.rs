rust
struct Binder(i32, i32, i32);

fn main() {
    let x = Binder(1, 2, 3);
    match x {
        Binder(_a, 1, 1) => {}
        Binder(_a, ..) => {}
    }
}
