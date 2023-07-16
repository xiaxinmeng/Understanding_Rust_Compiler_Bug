
impl<T> ~[T]: Buildable<T> {
    pure fn build(pusher: &pure fn(PushFn<T>)) -> ~[T] {
        let mut vec = ~[];
        do pusher |elem| {
            // unsafe is needed because this function should appear
            // pure from the outside.  We know it's safe because `vec`
            // does not escape, but the type system doesn't know it...

            unsafe { vec.push(elem); }
        }
        return vec;
    }
}
