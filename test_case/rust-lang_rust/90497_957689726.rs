rust
#[cfg(target_pointer_width = "32")]
fn foo(){
    // 32 bit impl
}

#[cfg(target_pointer_width = "64")]
fn foo(){
    // 64 bit impl
}


