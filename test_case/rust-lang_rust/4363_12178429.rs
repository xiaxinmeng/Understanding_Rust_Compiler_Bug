
struct A<T> {
    b: @B<T>
}
struct B<T> {
    a: @A<int>,
    b: @A<T>
}
