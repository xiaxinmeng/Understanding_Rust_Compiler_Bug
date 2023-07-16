
hello.rs:12:55: 12:67 error: the trait `core::marker::Sized` is not implemented for the type `[T]` [E0277]
hello.rs:12         unsafe { ::std::ptr::write(raw_place, value); ::Boxed::fin(place) }
                                                                  ^~~~~~~~~~~~
hello.rs:67:50: 67:59 note: in this expansion of box_! (defined in hello.rs)
hello.rs:12:55: 12:67 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:12:55: 12:67 note: `[T]` does not have a constant size known at compile-time
hello.rs:12:55: 12:67 note: required by `Boxed::fin`
hello.rs:9:25: 9:41 error: the trait `core::marker::Sized` is not implemented for the type `[T]` [E0277]
hello.rs:9         let mut place = ::BoxPlace::make();
                                   ^~~~~~~~~~~~~~~~
hello.rs:67:50: 67:59 note: in this expansion of box_! (defined in hello.rs)
hello.rs:9:25: 9:41 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:9:25: 9:41 note: `[T]` does not have a constant size known at compile-time
hello.rs:9:25: 9:41 note: required by `BoxPlace::make`
hello.rs:10:25: 10:41 error: the trait `Place<[_; 0]>` is not implemented for the type `BP<[T]>` [E0277]
hello.rs:10         let raw_place = ::Place::pointer(&mut place);
                                    ^~~~~~~~~~~~~~~~
hello.rs:67:50: 67:59 note: in this expansion of box_! (defined in hello.rs)
hello.rs:10:25: 10:41 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:10:25: 10:41 help: the following implementations were found:
hello.rs:10:25: 10:41 help:   <BP<T> as Place<T>>
hello.rs:10:25: 10:41 note: required by `Place::pointer`
hello.rs:78:18: 78:30 error: the trait `core::marker::Sized` is not implemented for the type `[T]` [E0277]
hello.rs:78                  ::Boxed::fin(place) }
                             ^~~~~~~~~~~~
hello.rs:78:18: 78:30 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:78:18: 78:30 note: `[T]` does not have a constant size known at compile-time
hello.rs:78:18: 78:30 note: required by `Boxed::fin`
hello.rs:74:25: 74:41 error: the trait `core::marker::Sized` is not implemented for the type `[T]` [E0277]
hello.rs:74         let mut place = ::BoxPlace::make();
                                    ^~~~~~~~~~~~~~~~
hello.rs:74:25: 74:41 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:74:25: 74:41 note: `[T]` does not have a constant size known at compile-time
hello.rs:74:25: 74:41 note: required by `BoxPlace::make`
hello.rs:75:25: 75:41 error: the trait `Place<[_; 0]>` is not implemented for the type `BP<[T]>` [E0277]
hello.rs:75         let raw_place = ::Place::pointer(&mut place);
                                    ^~~~~~~~~~~~~~~~
hello.rs:75:25: 75:41 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:75:25: 75:41 help: the following implementations were found:
hello.rs:75:25: 75:41 help:   <BP<T> as Place<T>>
hello.rs:75:25: 75:41 note: required by `Place::pointer`
