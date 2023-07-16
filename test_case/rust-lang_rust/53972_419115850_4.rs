rust
const<X> fn bar(f: const<X> fn()) { f() }
const<X> fn bar2(f: fn()) -> fn() { f }
const<X> fn bar3(f: const<X> fn()) -> fn() { f(); f }
const<X> fn bar4(f: const<X> fn()) -> const<X> fn() { f(); f }
