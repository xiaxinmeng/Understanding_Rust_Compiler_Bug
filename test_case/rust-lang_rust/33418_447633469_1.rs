rust
trait Tr: Send + !Sized {} //~ ERROR Negative trait bounds are not supported
