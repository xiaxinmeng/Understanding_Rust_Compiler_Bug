
#0  __rust_probestack () at /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.18/src/probestack.rs:55
#1  0x000000000050134a in std::sync::mutex::Mutex<T>::new (t=...) at /rustc/488381ce9ef0ceabe83b73127c659e5d38137df0/src/libstd/sync/mutex.rs:168
#2  0x0000000000501707 in <string_cache::atom::STRING_CACHE as core::ops::deref::Deref>::deref::__static_ref_initialize () at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/string_cache-0.7.3/src/atom.rs:46
#3  core::ops::function::FnOnce::call_once () at /rustc/488381ce9ef0ceabe83b73127c659e5d38137df0/src/libcore/ops/function.rs:227
#4  0x000000000050117c in lazy_static::lazy::Lazy<T>::get::{{closure}} () at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/inline_lazy.rs:31
#5  0x000000000050132e in std::sync::once::Once::call_once::{{closure}} () at /rustc/488381ce9ef0ceabe83b73127c659e5d38137df0/src/libstd/sync/once.rs:225
#6  0x00000000005325a8 in std::sync::once::Once::call_inner () at src/libstd/sync/once.rs:392
#7  0x00000000005012b3 in std::sync::once::Once::call_once (self=0x70aee8 <<string_cache::atom::STRING_CACHE as core::ops::deref::Deref>::deref::__stability::LAZY+32784>, f=...)
    at /rustc/488381ce9ef0ceabe83b73127c659e5d38137df0/src/libstd/sync/once.rs:225
#8  0x0000000000505afb in lazy_static::lazy::Lazy<T>::get (self=0x702ed8 <<string_cache::atom::STRING_CACHE as core::ops::deref::Deref>::deref::__stability::LAZY>, f=0x710060)
    at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/inline_lazy.rs:30
#9  <string_cache::atom::STRING_CACHE as core::ops::deref::Deref>::deref::__stability () at <::lazy_static::__lazy_static_internal macros>:16
#10 <string_cache::atom::STRING_CACHE as core::ops::deref::Deref>::deref (self=0x600e30) at <::lazy_static::__lazy_static_internal macros>:18
#11 0x00000000004d628c in <string_cache::atom::Atom<Static> as core::convert::From<alloc::borrow::Cow<str>>>::from (string_to_add=...) at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/string_cache-0.7.3/src/atom.rs:310
#12 0x00000000004d6875 in <string_cache::atom::Atom<Static> as core::convert::From<&str>>::from (string_to_add=...) at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/string_cache-0.7.3/src/atom.rs:323
#13 0x00000000004d5fb0 in segfaulthtml5evergolang::do_segfault (data=...) at src/lib.rs:32
#14 0x00000000004d5c9b in api_do_segfault (data_cstr=0x710040 "segfault") at src/lib.rs:9
#15 0x00000000004c9e10 in runtime.asmcgocall () at /nix/store/vdlp4402c4vdk86w74rk1njx5vicdlag-go-1.12.9/share/go/src/runtime/asm_amd64.s:635
#16 0x00007ffff5b1a8f8 in ?? ()
#17 0x00000000004c71b8 in runtime.goready.func1 () at /nix/store/vdlp4402c4vdk86w74rk1njx5vicdlag-go-1.12.9/share/go/src/runtime/proc.go:312
#18 0x000000c000000180 in ?? ()
#19 0x00000000004a5c40 in ?? () at /nix/store/vdlp4402c4vdk86w74rk1njx5vicdlag-go-1.12.9/share/go/src/runtime/proc.go:1082
#20 0x0000000000000027 in ?? ()
#21 0x0000000000020000 in ?? ()
#22 0x0000000000000000 in ?? ()
