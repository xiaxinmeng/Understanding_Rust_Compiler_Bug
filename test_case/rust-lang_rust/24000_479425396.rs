rust
#[derive(Clone)]
struct FunctionContainer {
    function: Box<dyn Function>,
}

trait Function: Fn(i32) -> i32 {
    fn clone_boxed(&self) -> Box<dyn Function>;
}

impl<T> Function for T
where
    T: 'static + Clone + Fn(i32) -> i32,
{
    fn clone_boxed(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Function> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}
