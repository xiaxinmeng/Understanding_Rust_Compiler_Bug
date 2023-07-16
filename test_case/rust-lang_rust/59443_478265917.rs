rust
cfg_match! {
    #[cfg(unix)] => { fn foo() { /* unix specific functionality */ } },
    #[cfg(target_pointer_width = "32")] => { fn foo() { /* non-unix, 32-bit functionality */ } }, 
    _ => { fn foo() { /* fallback implementation */ } }
}
