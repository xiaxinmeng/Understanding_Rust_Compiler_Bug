rust
const fn foo() {}
const FOO: const fn() = foo;
const fn bar() { FOO() }
const fn baz(x: const fn()) { x() }
const fn bazz() { baz(FOO) }
