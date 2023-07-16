Rust
#![feature(const_async_blocks)]
#![feature(unsized_locals)]

struct T {
    a: [(); {
        let y = async {
            let f: (dyn for<'a> std::future::Future<Output = ()> + Unpin);
            
            (&mut f).await
        };
        
        core::mem::forget(y);
        1
    }]
}
