rust
fn main() {
    println!("{}, {}", id1(-1), id2(-1))
}
fn id1(mut a: i32) -> i64 {
    let mut out = 0;
    while a != 0 {
        out += 1;
        a -= 1;
    }
    out
}

fn id2(a: i32) -> i64 {
    if a == 0 {
        0
    } else {
        1 + id2(a - 1)
    }
}

