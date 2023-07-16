rust
struct Foo {
    field: u8,
}

struct Bar {
    foo: Foo,
}

mod lock {
    pub struct Lock<T> {
        data: T,
    }
    
    impl<T> std::ops::Deref for Lock<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }    
}

fn main() {
    let lock = lock::Lock {
        data: Bar {
            foo: Foo { field: 0 },
        }
    };
    
    let _ = lock.field; // ERROR
    // ^ but rustc suggests to use `lock.data.foo.field` when `data` is private
    
    // Unrelated note: the error on the following line could be improved
    // by pointing into the `Deref::deref` method ?
    // let _ = lock.foo.field;
}
