rust
struct S;

pub fn f() {
    loop {
        let x;
        if true {
            x = S;
        }
        drop(x);
    }
}

fn main() {}
