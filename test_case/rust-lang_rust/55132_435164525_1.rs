rust
imp::Thread::new(stack_size, transmute::<Box<FnBox()>, Box<FnBox()>>(Box::new(main)))
