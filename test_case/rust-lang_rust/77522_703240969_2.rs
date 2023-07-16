rust
    trait Trait {
        fn f() -> Pin<Box<dyn Future<Output = ()> + Send>> {
            #[allow(clippy::missing_docs_in_private_items)]
            async fn __f() {
                ... // original body
            }
            Box::pin(__f())
        }
    }
    