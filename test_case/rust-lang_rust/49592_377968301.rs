rust
#[macro_export]
macro_rules! async_block {
    ($e:expr) => {
        ::futures::__rt::gen(unsafe {
                    static move || {
                        if false {
                            yield ::futures::Async::NotReady
                        }
                        {
                        $e
                        }
                    }
                }
                )
    }

}
