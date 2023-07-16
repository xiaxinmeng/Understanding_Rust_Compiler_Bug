
pub trait Foo {
    async fn foo(self: &(impl std::ops::Deref<Target=Self> + Clone + Send + 'static));
}

pub struct FooImpl {
}

impl Foo for FooImpl {
    async fn foo(self: &(impl std::ops::Deref<Target=Self> + Clone + Send + 'static)) {
    }
}

impl FooImpl {
    pub async fn test() {
        Arc::new(FooImpl {}).foo().await;
    }
}
