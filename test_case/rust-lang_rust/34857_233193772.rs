 rust
// Call as `fs::read::<String>("my_file")`. Alternatively, we could try landing default type arguments :)
mod fs {
    fn read<T: SomeTrait, P: AsRef<Path>>(path: P) -> T { /* ... */ }
}
