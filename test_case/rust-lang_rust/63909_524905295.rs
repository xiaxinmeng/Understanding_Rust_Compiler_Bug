rust
extern "C" fn bar() -> ! { panic!("lolz") }
extern "C" fn baz() -> i32 {
    if let Ok(_) = std::panic::catch_unwind(|| bar() ) {
        unsafe { std::hint::unreachable_unchecked() }
    }
    42
}
fn main() { std::process::exit((baz() != 42) as i32); }
