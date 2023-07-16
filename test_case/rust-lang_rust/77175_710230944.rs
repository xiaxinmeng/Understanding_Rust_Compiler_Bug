rust
#[deny(single_use_lifetimes)]
async fn bar<'a>(s1: String, s2: &'_ str, s3: &'a str) -> String {
    s3.to_owned()
}
