 rust
Program received signal SIGSEGV, Segmentation fault.
0x7f55e530 in rand::cell::Cell<T>::get (self=0x1) at ../src/libcore/cell.rs:195
195     ../src/libcore/cell.rs: No such file or directory.
(gdb) bt
#0  0x7f55e530 in rand::cell::Cell<T>::get (self=0x1) at ../src/libcore/cell.rs:195
#1  0x7f55e508 in rand::rc::RcBoxPtr::strong<alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>> (self=0xb6ff84c8) at ../src/liballoc/rc.rs:875
#2  0x7f55e290 in rand::rc::RcBoxPtr::inc_strong<alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>> (self=0xb6ff84c8) at ../src/liballoc/rc.rs:880
#3  0x7f55e218 in rand::rc::Rc<T>.Clone::clone (self=0xb6ff84c8) at ../src/liballoc/rc.rs:491
#4  0x7f55e83c in fnfn (t=0xb6ff84c8) at /home/odroid/.cargo/registry/src/github.com-48ad6e4054423464/rand-0.3.13/src/lib.rs:884
#5  0x7f55e64c in rand::thread::local::LocalKey<T>::with<closure,alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>> (self=0x7f5a67bc <thread_rng::THREAD_RNG_KEY::h82deed7a23a1423axgf>, f=...)
    at ../src/libstd/thread/local.rs:191
#6  0x7f55e574 in rand::thread_rng () at /home/odroid/.cargo/registry/src/github.com-48ad6e4054423464/rand-0.3.13/src/lib.rs:884
#7  0x7f5590c0 in test_temp::main () at src/main.rs:5
#8  0x7f5719bc in sys_common::unwind::try::try_fn::h15347629866114009016 ()
#9  0x7f56efe0 in __rust_try ()
#10 0x7f571528 in rt::lang_start::hc182d4243c0ca773kmy ()
#11 0x7f559964 in main ()
