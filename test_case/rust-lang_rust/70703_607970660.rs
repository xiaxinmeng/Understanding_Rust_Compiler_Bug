rust
fn make_product_consumer<T,F: Factory<Product=T>>(f: F) -> impl ProductConsumer<T> {
    ()
}
