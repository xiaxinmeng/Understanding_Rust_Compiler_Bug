diff
fn assert_service() -> impl Service {
-   ServiceFromAsyncFn(MyFn, PhantomData)
+   ServiceFromAsyncFn(MyFn, PhantomData::<String>)
}
