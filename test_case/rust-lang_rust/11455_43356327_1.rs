
struct MyTransformResult<T>(SomeConcreteType<T>);
impl<T> Iterator<T> for MyTransformResult<T> { ... }

fn my_transform<T, Iter: Iterator<T>>(iter: Iter) -> MyTransformResult<T> { ... }
