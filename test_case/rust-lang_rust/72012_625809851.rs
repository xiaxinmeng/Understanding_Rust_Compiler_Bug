rust
#[target_feature(enable="avx")]
unsafe fn use_avx() {
    println!("Hello from AVX")
}

fn call_it(f: impl FnOnce()) {
    f();
}

fn main() {
    let x = use_avx;
}
