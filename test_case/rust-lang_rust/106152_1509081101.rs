
enum State<T, F> {
    Uninit(F),
    Init(T),
    Poisoned,
}
