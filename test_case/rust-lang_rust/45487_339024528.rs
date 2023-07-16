 rust
fn main() {
    extern "C" {
        // C signature: int __ctzdi2(long x)
        fn __ctzdi2(x: i64) -> i32;
    }

    // you may need to test different inputs to trigger the bug (if there's one)
    println!("{}", unsafe { __ctzdi2(0) });
}
