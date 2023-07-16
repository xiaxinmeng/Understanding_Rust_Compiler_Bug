rust
    #![feature(optin_builtin_traits, dynsized)]
    use core::marker::DynSized;
    unsafe auto trait Auto: ?DynSized {}
    