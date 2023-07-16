
trait Drop {
    fn finalize() { /* default impl */ }
}
trait NonCopyable: Drop;

// Here I'm using imaginary syntax to indicate that MyType impls both NonCopyable and all the supertraits of NonCopyable
impl MyType: NonCopyable(*);
