 rust
enum T {
    On,
    Off
}
enum Repeats {
    Zero
    One(T),
    Two(T,T),
    Three(T,T,T)
    Four(T,T,T,T)
    Five(T,T,T,T,T)
}
