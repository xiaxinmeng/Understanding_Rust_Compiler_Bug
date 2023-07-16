Rust
#![feature(unsized_locals)]

struct T {
    a: [(); {
        let y = async {
            let f: (dyn for<'a> std::future::Future<Output = ()> + Unpin);
            
            f.await
        };
        
        core::mem::forget(y);
        1
    }]
}
