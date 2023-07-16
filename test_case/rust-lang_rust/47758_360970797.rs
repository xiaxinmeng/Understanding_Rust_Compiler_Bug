
#![feature(generators)]

enum Test{
    A(i32),
    B
}

fn main() {
    let test = Test::A(0);
    let _ = move || {
        if let Test::A(ref _a) = test {
            yield 0;
        }
    };
}

