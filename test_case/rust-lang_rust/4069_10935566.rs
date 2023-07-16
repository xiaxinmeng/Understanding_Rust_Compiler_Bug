
trait Drop {
    fn finalize(&self);
}

trait NonCopyable;

impl <T: NonCopyable> T: Drop {
    fn finalize(&self) { }
}
