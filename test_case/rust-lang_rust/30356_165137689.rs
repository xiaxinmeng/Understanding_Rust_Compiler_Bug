 Rust
mod a { mod b { fn f() -> i32 { 3 } } }

fn g1() -> i32 {
    a.b.f()
}

fn g2() -> i32 {
    a::b.f()
}

fn g3() -> i32 {
    a::b()
}

fn g4() -> i32 {
    a::b
}

fn main() {
    println!("g1: {} g2: {} g3: {}", g1(), g2(), g3());
}
