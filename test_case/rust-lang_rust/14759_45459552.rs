 rust
println!("Twiddling the frobs with {}", foo);
match foo.upgrade() {
    Some(bar) => twiddleFrobsWith(bar),
    None => reportNoFrobsToTwiddle()
}
