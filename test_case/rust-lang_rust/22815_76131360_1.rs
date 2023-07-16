
foo.rs:15:5: 15:26 error: the trait `core::marker::Send` is not implemented for the type `<usize as Foo>::A` [E0277]
foo.rs:15     is_send::<Bar<usize>>();
              ^~~~~~~~~~~~~~~~~~~~~
foo.rs:15:5: 15:26 note: `<usize as Foo>::A` cannot be sent between threads safely
foo.rs:15     is_send::<Bar<usize>>();
              ^~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
