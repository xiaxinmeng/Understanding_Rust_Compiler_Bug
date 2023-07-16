
type PushFn<T> = &fn(+t: T);
trait Buildable<T> {
    pure fn build(pusher: &pure fn(PushFn<T>)) -> self;
}
