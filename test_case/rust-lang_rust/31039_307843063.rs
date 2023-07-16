rust
trait Test {
    fn test(&self) -> Option<Self>;
    //                ~~~~~~~~~~~~
    //            Incorrectly permitted before.
}
