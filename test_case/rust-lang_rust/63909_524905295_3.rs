rust
// main.rs
extern "C" { fn foo(x: extern "C" fn() -> !) -> !; }
extern "C" fn bar() -> ! { panic!("lolz") }
extern "C" fn baz() -> i32 {
    if let Ok(_) = std::panic::catch_unwind(|| unsafe { foo(bar) }) {
        unsafe { std::hint::unreachable_unchecked() }
    }
    42
}
fn main() {
   std::process::exit((baz() != 42) as i32);
}
