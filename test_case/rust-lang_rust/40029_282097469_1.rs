rust
    #[repr(u8)]
    enum Flag<T> {
        Alive(T),
        Dropped(u8),
    }
