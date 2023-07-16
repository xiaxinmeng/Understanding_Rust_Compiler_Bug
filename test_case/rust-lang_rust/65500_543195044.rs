rust
trait MyTrait {
    type MyType;
    fn call_boxed<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Self::MyType> + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
struct MyStruct;
impl MyTrait for MyStruct {
    type MyType = i32;
    fn call_boxed<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Self::MyType> + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        #[allow(clippy::used_underscore_binding)]
        async fn __call_boxed(_self: &MyStruct) -> <MyStruct as MyTrait>::MyType {
            _self.call().await
        }
        Box::pin(__call_boxed(self))
    }
}
impl MyStruct {
    async fn call(&self) -> Self::Output {
        5
    }
}
fn main() {}
