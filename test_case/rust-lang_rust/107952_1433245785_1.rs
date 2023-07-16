rust
    fn update<P, E>()
    where
        P: Protocol<Self::Event, Effect = E>,
        E: Composite;
