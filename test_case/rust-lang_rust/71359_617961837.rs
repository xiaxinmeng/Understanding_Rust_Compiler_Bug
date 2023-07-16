rust
        impl<T: Send> Events<T> {
            pub fn iter(&self) -> impl Iterator<Item = (&'static str, &T)> {
                iter::empty()
                $(
                    .chain(
                        self.$name.iter()
                        .map(|value| (stringify!($name), value))
                    )
                )*
            }
