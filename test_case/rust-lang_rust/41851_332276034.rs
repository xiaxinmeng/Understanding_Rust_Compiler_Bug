rust
fn main() {
    let a = "Hello".to_owned();
    
    loop {
        let a = a.clone(); // clone the object to create a copy that can be consumed 
                                  //   ... by the closure or use a Rc/Arc pointer.
        let test = move || {
            consume(a);
        };
    }
}

fn consume<T>(_: T) {}
