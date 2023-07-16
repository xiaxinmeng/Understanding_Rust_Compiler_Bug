rust
macro_rules! with_doc {
    ($doc: expr) => {
        #[doc = $doc]
        async fn f() {} 
    };
}

with_doc!(concat!(""));
