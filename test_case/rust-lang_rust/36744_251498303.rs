 rust
struct A<'a> {
    a: &'a i32,
}

fn call<T>(s: T, functions: &Vec<fn(&T)>) {
}

fn f(a: &A) { }

fn main() {
    let a = A { a: &10 };

    let vec: Vec<fn(&A)> = vec![f];
    call(a, &vec); // 'x
}
