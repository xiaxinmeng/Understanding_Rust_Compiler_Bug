
use std::sync::Arc;

trait GenFn {
    type Output: Fn() -> String;
    
    fn gen(&mut self) -> Self::Output;
}

struct FnBuilder;

impl GenFn for FnBuilder {
    type Output = Arc<dyn Fn() -> String>;
    
    fn gen(&mut self) -> Self::Output {
        Arc::new(|| "Hello World!".to_string())
    }
}
