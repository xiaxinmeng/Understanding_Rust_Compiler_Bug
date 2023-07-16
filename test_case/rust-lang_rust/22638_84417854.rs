 rust
extern crate threadpool;

use std::sync::{Arc,Mutex};
use threadpool::ScopedPool;

fn test<'pool, F: FnMut(usize) + Send + Sync + 'pool>(mut callback: F, depth: usize, pool: Arc<Mutex<ScopedPool<'pool>>>) {
    if depth == 0 {
        callback(100);
    } else {
        struct Foo<F: FnMut(usize)> {
            callback: F
        }

        let foo = Arc::new(Mutex::new(Foo{callback: move |u| {
            callback(u);
        }}));

        for _ in 0..50 {
            struct Bar<F: FnMut(usize)> {
                callback: F
            }

            let foo = foo.clone();

            let bar = Arc::new(Mutex::new(Bar {
                callback: move |u| {
                    let foo = &mut *foo.lock().unwrap();

                    (foo.callback)(u);
                }
            }));

            for _ in 0..5 {
                let descending_pool = pool.clone();
                let pool = pool.lock().unwrap();
                let bar = bar.clone();
                pool.execute(move || {
                    test(move |n| {
                        let bar = &mut *bar.lock().unwrap();

                        (bar.callback)(n);
                    }, depth - 1, descending_pool);
                });
            }
        }
    }
}

fn main() {
    let pool = Arc::new(Mutex::new(ScopedPool::new(4)));

    test(|num| {
        println!("woohoo {}", num);
    }, 1, pool);
}
