
csguest@itl:~$ rustc foo.rs
foo.rs:8:2: 8:47 error: the trait `core::clone::Clone` is not implemented for the type `core::atomic::AtomicBool`
foo.rs:8    assert_clone::<foo::core::atomic::AtomicBool>();
            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
