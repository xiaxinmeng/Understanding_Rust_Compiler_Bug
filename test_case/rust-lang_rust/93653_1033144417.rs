rs
trait BoxTake<T> {
    fn take_(this: Box<T>) -> (T, Box<MaybeUninit<T>>);
    fn initialize(this: Box<MaybeUninit<T>>, value: T) -> Box<T>;
}

impl<T> BoxTake<T> for Box<T> {
    fn take(this: Box<T>) -> (T, Box<MaybeUninit<T>>) {
        unsafe {
            let ptr = Box::into_raw(this);
            let value = ptr.read();
            (value, Box::from_raw(ptr.cast()))
        }
    }

    fn replace(mut this: Box<MaybeUninit<T>>, value: T) -> Box<T> {
        unsafe {
            this.write(value);
            Box::from_raw(Box::into_raw(this).cast())
        }
    }
}

fn main() {
    let boxed = Box::new(42);
    let (fourty_two, allocation) = Box::take(boxed);
    assert_eq!(fourty_two, 42);

    let initialized = Box::replace(allocation, 7);
    assert_eq!(initialized, 7);
}
