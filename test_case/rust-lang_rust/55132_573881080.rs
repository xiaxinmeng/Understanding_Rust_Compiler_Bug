rust
let rc = Rc::new(1);
let handle = unsafe { thread::spawn_unchecked(move || assert_eq!(*rc, 1)) };
handle.join().unwrap();
