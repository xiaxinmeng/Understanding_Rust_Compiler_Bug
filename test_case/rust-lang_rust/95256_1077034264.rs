rust
 const _: fn() = || {

            trait AmbiguousIfImpl<A> {
                fn some_item() {}
            }

            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}

            impl<T: ?Sized + UnwindSafe> AmbiguousIfImpl<i32> for T {}

            let _ = <io::Error as AmbiguousIfImpl<_>>::some_item;
        };
    };
