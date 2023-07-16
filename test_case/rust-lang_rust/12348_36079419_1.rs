
struct Cons<T>
{
    head: T,
    tail: Option<~Cons<T>>
}

type List<T> = Option<~Cons<T>>;
