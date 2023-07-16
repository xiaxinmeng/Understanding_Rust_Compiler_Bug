plain
   Compiling rustc-demangle v0.1.21
error[E0308]: mismatched types
    --> library/alloc/src/boxed.rs:2206:57
     |
2206 |                 let raw: *mut dyn Error = Box::into_raw(self);
     |                                                         ^^^^ expected struct `Box`, found trait object `dyn core::error::Error`
     |
     = note:    expected struct `Box<dyn core::error::Error, _>`
             found trait object `(dyn core::error::Error + 'static)`
     = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
     |
2206 |                 let raw: *mut dyn Error = Box::into_raw(Box::new(self));
     |                                                         +++++++++    +
error[E0308]: mismatched types
    --> library/alloc/src/boxed.rs:2210:17
     |
2210 |             Err(self)
2210 |             Err(self)
     |                 ^^^^ expected struct `Box`, found trait object `dyn core::error::Error`
     |
     = note:    expected struct `Box<dyn core::error::Error>`
             found trait object `(dyn core::error::Error + 'static)`
     = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
     |
2210 |             Err(Box::new(self))
     |                 +++++++++    +
error[E0308]: mismatched types
    --> library/alloc/src/boxed.rs:2222:35
     |
     |
2222 |         let err: Box<dyn Error> = self;
     |                  --------------   ^^^^ expected struct `Box`, found trait object `dyn core::error::Error`
     |                  expected due to this
     |
     |
     = note:    expected struct `Box<dyn core::error::Error>`
             found trait object `(dyn core::error::Error + Send + 'static)`
     = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
     |
2222 |         let err: Box<dyn Error> = Box::new(self);
     |                                   +++++++++    +
error[E0308]: mismatched types
    --> library/alloc/src/boxed.rs:2223:31
     |
     |
2223 |         <dyn Error>::downcast(err).map_err(|s| unsafe {
     |                               ^^^ expected trait object `dyn core::error::Error`, found struct `Box`
     |
     = note: expected trait object `(dyn core::error::Error + 'static)`
                      found struct `Box<dyn core::error::Error>`
error[E0308]: mismatched types
    --> library/alloc/src/boxed.rs:2237:35
     |
     |
2237 |         let err: Box<dyn Error> = self;
     |                  --------------   ^^^^ expected struct `Box`, found trait object `dyn core::error::Error`
     |                  expected due to this
     |
     |
     = note:    expected struct `Box<dyn core::error::Error>`
             found trait object `(dyn core::error::Error + Send + Sync + 'static)`
     = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
     |
2237 |         let err: Box<dyn Error> = Box::new(self);
     |                                   +++++++++    +
error[E0308]: mismatched types
    --> library/alloc/src/boxed.rs:2238:31
     |
     |
2238 |         <dyn Error>::downcast(err).map_err(|s| unsafe {
     |                               ^^^ expected trait object `dyn core::error::Error`, found struct `Box`
     |
     = note: expected trait object `(dyn core::error::Error + 'static)`
                      found struct `Box<dyn core::error::Error>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
