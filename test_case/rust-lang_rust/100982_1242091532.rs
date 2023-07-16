plain
   Compiling thread_local v1.1.4
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling annotate-snippets v0.9.1
error[E0277]: the trait bound `Mutex<Vec<&'static (dyn Callsite + 'static)>>: Default` is not satisfied
    |
    |
257 | static LOCKED_CALLSITES: Lazy<Mutex<Vec<&'static dyn Callsite>>> = Lazy::new(Default::default);
    |                                                                              ^^^^^^^^^^^^^^^^ the trait `~const Default` is not implemented for `Mutex<Vec<&'static (dyn Callsite + 'static)>>`
    |
note: the trait `Default` is implemented for `Mutex<Vec<&'static (dyn Callsite + 'static)>>`, but that implementation is not `const`
    |
    |
257 | static LOCKED_CALLSITES: Lazy<Mutex<Vec<&'static dyn Callsite>>> = Lazy::new(Default::default);


error[E0277]: the trait bound `RwLock<Vec<Registrar>>: Default` is not satisfied
    |
522 |         Lazy::new(Default::default);
522 |         Lazy::new(Default::default);
    |                   ^^^^^^^^^^^^^^^^ the trait `~const Default` is not implemented for `RwLock<Vec<Registrar>>`
    |
note: the trait `Default` is implemented for `RwLock<Vec<Registrar>>`, but that implementation is not `const`
    |
522 |         Lazy::new(Default::default);
    |                   ^^^^^^^^^^^^^^^^

