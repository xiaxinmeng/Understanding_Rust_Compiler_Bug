rust
macro_rules! copy_doc {
    ($(#[$m:meta])* $(cfg $c:tt $i:item)+) => {
        $($(#[$m])* #[cfg $c] $i)+
    };
}

copy_doc! {
    /// Some doc for foo regardless of feature.
    cfg(feature = "bar")
    pub fn foo(x: Bar);
    cfg(feature = "baz")
    pub fn foo(x: Baz, y: Baz) -> Baz;
}
