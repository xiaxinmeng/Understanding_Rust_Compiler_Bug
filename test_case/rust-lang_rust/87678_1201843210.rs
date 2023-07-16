rust
macro_rules! extern_item {
    ($($toks: tt)+) => {
        extern "C" $($toks)+
    };
}

// Use `sysv64` on x86_64 targets instead of C
#[cfg(target_arch = "x86_64")]
macro_rules! extern_item {
    ($($toks: tt)+) => {
        extern "sysv64" $($toks)+
    };
}

extern_item! { {
    fn extern_declaration();
} }

extern_item! { fn definition() { ... } }
