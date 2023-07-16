rs
trait Private {
}

unsafe impl<T: Private> Send for T {
}
