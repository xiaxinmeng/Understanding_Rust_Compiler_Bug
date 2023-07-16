 java
#0  0x7f55e4f8 in core::cell::{{impl}}::get<usize> (self=0x1)
    at ..src/libcore/cell.rs:195
#1  0x7f55e4d0 in alloc::rc::RcBoxPtr::strong<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>,alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>> (self=0xb6ff84c8)
    at ..src/liballoc/rc.rs:875
#2  0x7f55e1f8 in alloc::rc::RcBoxPtr::inc_strong<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>,alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>> (self=0xb6ff84c8)
    at ..src/liballoc/rc.rs:880
#3  0x7f55e160 in alloc::rc::{{impl}}::clone<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>> (
    self=0xb6ff84c8) at ..src/liballoc/rc.rs:490
#4  0x7f55e6a8 in rand::thread_rng::{{closure}} (t=0xb6ff84c8)
    at .cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.3.14/src/lib.rs:884
#5  0x7f55e5e4 in std::thread::local::{{impl}}::with<alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>,closure,alloc::rc::Rc<core::cell::RefCell<rand::reseeding::ReseedingRng<rand::StdRng, rand::ThreadRngReseeder>>>> (
    self=0x7f598dfc <rand::thread_rng::THREAD_RNG_KEY::h8711901663c7347a>, f=...)
    at ..src/libstd/thread/local.rs:211
#6  0x7f55e51c in rand::thread_rng () at .cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.3.14/src/lib.rs:884
#7  0x7f559048 in rand_test::main () at src/main.rs:5
#8  0x7f56f03c in std::panicking::try::call::h94b205a774fa265c ()
#9  0x7f573e4c in __rust_try ()
#10 0x7f573de0 in __rust_maybe_catch_panic ()
#11 0x7f56ea90 in std::rt::lang_start::ha9a21237c3a23329 ()
#12 0x7f5598f4 in main ()
