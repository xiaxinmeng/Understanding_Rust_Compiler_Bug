rust
struct Bar(const fn());
const fn bar(f: Bar) {
    (f.0)() // not allowed
}
