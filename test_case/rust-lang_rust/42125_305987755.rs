rust
mod m {
    struct Priv;
    pub struct Pub<T>(pub T);

    pub type PubAlias = Priv;
    pub type PubAlias2 = Pub<Priv>;
}
