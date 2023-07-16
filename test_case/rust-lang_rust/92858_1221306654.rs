rust
struct S;

fn main() {
    let mut cs = S{};
    let mut flag = false;
    for i in 1..3 {
        if !flag {
            cs = S{};
            flag = true;
        }
        if flag {
            let t = cs;
            flag = false;
        }
    }
}
