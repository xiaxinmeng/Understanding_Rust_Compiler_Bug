plain
   Compiling rand v0.7.3
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling core v0.0.0 (/checkout/library/core)
error[E0152]: found duplicate lang item `env_set_var`
    |
    |
348 | pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    |
    |
    = note: the lang item is first defined in crate `std` (which `std` depends on)
    = note: first definition in `std` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-12b9039cc0378704.so, /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-12b9039cc0378704.rlib
    = note: second definition in the local crate (`std`)

error[E0152]: found duplicate lang item `env_remove_var`
    |
    |
391 | pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    |
    |
    = note: the lang item is first defined in crate `std` (which `std` depends on)
    = note: first definition in `std` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-12b9039cc0378704.so, /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-12b9039cc0378704.rlib
    = note: second definition in the local crate (`std`)
For more information about this error, try `rustc --explain E0152`.
error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:19:00
