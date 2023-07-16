
// Represents anything that can generate a string
pub(crate) trait ErrorMessageProvider<TTarget>
{
    fn provide(&self, target: &TTarget) -> String;
}

// Closure that returns a string
impl<TTarget, TFunction: Fn(&TTarget) -> String>
    ErrorMessageProvider<TTarget> for TFunction
{
    fn provide(&self, target: &TTarget) -> String {
        self(target)
    }
}

// A string
impl<TTarget> ErrorMessageProvider<TTarget> for &str
{
    fn provide(&self, _target: &TTarget) -> String {
        (*self).to_owned()
    }
}

// Function that takes a String, or a closure that returns a string
fn take<TTarget>(provider: impl ErrorMessageProvider<TTarget>, target: TTarget) {
    
}

fn main() {
    take("BAR", 100); // Works
    take(|x| format!("{:?}", x), 100); // Error: implementation of `FnOnce` is not general enough
}
