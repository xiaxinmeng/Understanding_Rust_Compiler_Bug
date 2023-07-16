
libstd\sys\windows\thread_local.rs:247:42: 247:51 error: the trait `core::clone::Clone` is not implemented for the type `unsafe extern "C" fn(*mut u8)` [E0277]
libstd\sys\windows\thread_local.rs:247                 (*DTORS).iter().cloned().collect()
