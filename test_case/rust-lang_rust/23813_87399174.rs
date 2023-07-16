
[18:01:26] <Luqman> steveklabnik: for https://github.com/rust-lang/rust/pull/23813, not all &T are marked noalias just those for which T contains no UnsafeCell<U>
