
#0  0x00005555555c52b0 in compiler_builtins::probestack::__rust_probestack ()
    at src/rustc/compiler_builtins_shim/../../libcompiler_builtins/src/probestack.rs:55
#1  0x000055555555dbbe in lazy_static::lazy::{{impl}}::get::{{closure}}<[(&str, usize); 767],fn() -> [(&str, usize); 767]>
    () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-0.2.8/src/lazy.rs:22
#2  0x000055555555d82a in std::sync::once::{{impl}}::call_once::{{closure}}<closure> ()
    at ~/src/rust/src/libstd/sync/once.rs:227
#3  0x0000555555580d6d in std::sync::once::Once::call_inner (
    self=0x5555557f51d0 <<x11_dl::xlib::Xlib::init::SYMS as core::ops::deref::Deref>::deref::__stability::LAZY+8>, 
    ignore_poisoning=<Fehler beim Lesen der Variable: access outside bounds of object referenced via synthetic pointer>, 
    init=...) at src/libstd/sync/once.rs:307
#4  0x000055555555d6cc in std::sync::once::Once::call_once<closure> (
    self=0x5555557f51d0 <<x11_dl::xlib::Xlib::init::SYMS as core::ops::deref::Deref>::deref::__stability::LAZY+8>, f=...)
    at ~/src/rust/src/libstd/sync/once.rs:227
#5  0x000055555557dc00 in lazy_static::lazy::Lazy<[(&str, usize); 767]>::get<[(&str, usize); 767],fn() -> [(&str, usize); 767]> (self=<optimized out>)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-0.2.8/src/lazy.rs:22
#6  x11_dl::xlib::{{impl}}::init::{{impl}}::deref::__stability () at <__lazy_static_internal macros>:20
#7  x11_dl::xlib::{{impl}}::init::{{impl}}::deref (self=0x5555555c9743 <str.eC>) at <__lazy_static_internal macros>:21
#8  0x000055555557d532 in x11_dl::xlib::Xlib::init (self=0x7ffffffef940)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/x11-dl-2.14.0/src/link.rs:48
#9  0x000055555557d99d in x11_dl::xlib::Xlib::open ()
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/x11-dl-2.14.0/src/link.rs:61
#10 0x000055555555b548 in c::foo () at src/main.rs:10
#11 0x000055555555b9ed in c::main () at src/main.rs:18
