rust
#![feature(const_fn)]

struct Test(());

const fn new() -> Test { Test(()) }

static FOO: Test = new();
