let foo = &UnsafeCell::new(Vec::new());
let bar = &*foo;
unsafe { (*foo.get()).push(1); }
unsafe { (*bar.get()).push(2); }
unsafe { assert_eq!((*foo.get()).len(), 2); }
