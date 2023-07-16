 rust
fn safe_forget<T>(data: T) {
    use std::rc::Rc;
    use std::cell::RefCell;

    struct Leak<T> {
        cycle: RefCell<Option<Rc<Rc<Leak<T>>>>>,
        data: T,
    }

    let e = Rc::new(Leak {
        cycle: RefCell::new(None),
        data: data,
    });
    *e.cycle.borrow_mut() = Some(Rc::new(e.clone())); // Create a cycle
}

fn main() {
    struct Foo<'a>(&'a mut i32);

    impl<'a> Drop for Foo<'a> {
        fn drop(&mut self) {
            *self.0 += 1;
        }
    }

    let mut data = 0;

    {
        let foo = Foo(&mut data);
        safe_forget(foo);
    }

    data += 1; // super make sure there's no outstanding borrows

    println!("{:?}", data); // prints 1, should print 2
}

