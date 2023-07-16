rust
fn non_secure_callback(callback: fn(i32) -> i32) {
    let val = (#[cmse_nonsecure_call] callback)(42);
}
