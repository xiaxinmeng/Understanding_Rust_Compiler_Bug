rust
trait ModuleConfigSpec<T>
where for<'a> {
    Self: ModuleConfig<Type2<'a> = &'a T>,
    <Self as ModuleConfig>::Type1: 'a,
} {}
