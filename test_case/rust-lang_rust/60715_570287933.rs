rust
    lazy_static! {
        static ref THIS: RwLock<Cell<
            &Cell<i32> // something not Send
        >> = RwLock::new(Cell::new(
            Box::leak(Box::new(Cell::new(0)))
        ));
    }

    fn main ()
    {
        let first_ref: &Cell<i32> = THIS.read().unwrap().get();
        ::std::thread::spawn(|| {
            let second_ref: &Cell<i32> = THIS.read().unwrap().get();
            loop { second_ref.set(0); }
        });
        loop { first_ref.set(!0); }
    }
    