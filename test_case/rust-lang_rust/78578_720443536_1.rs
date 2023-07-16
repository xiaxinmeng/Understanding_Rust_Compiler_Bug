rust
const FOO: NotAMutex<&i32> = NotAMutex(UnsafeCell::new(&{
    let x = 42;
    x
}));
