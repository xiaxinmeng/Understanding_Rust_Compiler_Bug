rust
macro_rules! doc_comment {
    ($x: expr, $($tt: tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! paged_routes {
     ($($rest: tt)* ) => {
         doc_comment! {
            concat!("", ""), 
            async fn name() {} 
        } 
        paged_routes! {
            $($rest)* 
        } 
    }; 
    () => {} 
}
impl Mastodon {
    paged_routes! {
    }
}

