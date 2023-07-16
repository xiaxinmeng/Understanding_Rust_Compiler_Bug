rust
pub mod foo {
    struct Private;
    
    mod internal {
        pub type Bar = Vec<super::Private>;
    }
}
