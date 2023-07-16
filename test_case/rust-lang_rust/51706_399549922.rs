rust
unsafe fn dont_call_me() -> ! { 
    std::mem::transmute(()) 
}

fn main() {
    unsafe { dont_call_me() }
}
