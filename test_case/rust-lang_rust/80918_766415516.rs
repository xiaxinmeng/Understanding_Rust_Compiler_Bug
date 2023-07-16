rust
        /// When return value overflows (i.e., `self > (1 << (N-1))` for type
        /// `uN`), it panics in debug mode and return value is wrapped to 0 in
        /// release mode (the only situation in which method can return 0).
