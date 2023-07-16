rust
// instead of:
impl<A> Eq for (A) where    A: Eq + ?Sized
impl<A, B> Eq for (A, B) where    A: Eq,    B: Eq + ?Sized
impl<A, B, C> Eq for (A, B, C) where    A: Eq,    B: Eq,    C: Eq + ?Sized
impl<A, B, C, D> Eq for (A, B, C, D) where    A: Eq,    B: Eq,    C: Eq,    D: Eq + ?Sized
impl<A, B, C, D, E> Eq for (A, B, C, D, E) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq + ?Sized
impl<A, B, C, D, E, F> Eq for (A, B, C, D, E, F) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq,    F: Eq + ?Sized
impl<A, B, C, D, E, F, G> Eq for (A, B, C, D, E, F, G) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq,    F: Eq,    G: Eq + ?Sized
impl<A, B, C, D, E, F, G, H> Eq for (A, B, C, D, E, F, G, H) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq,    F: Eq,    G: Eq,    H: Eq + ?Sized
impl<A, B, C, D, E, F, G, H, I> Eq for (A, B, C, D, E, F, G, H, I) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq,    F: Eq,    G: Eq,    H: Eq,    I: Eq + ?Sized
impl<A, B, C, D, E, F, G, H, I, J> Eq for (A, B, C, D, E, F, G, H, I, J) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq,    F: Eq,    G: Eq,    H: Eq,    I: Eq,    J: Eq + ?Sized
impl<A, B, C, D, E, F, G, H, I, J, K> Eq for (A, B, C, D, E, F, G, H, I, J, K) where    A: Eq,    B: Eq,    C: Eq,    D: Eq,    E: Eq,    F: Eq,    G: Eq,    H: Eq,    I: Eq,    J: Eq,    K: Eq + ?Sized

// just:
impl<*> Eq for (*) where *: Eq + ?Sized
