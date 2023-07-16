rust
const fn bar(f: const fn()) { f() }
const fn bar2(f: fn()) -> fn() { f }
const fn bar3(f: const fn()) -> fn() { f(); f }
const fn bar4(f: const fn()) -> const fn() { f(); f }
