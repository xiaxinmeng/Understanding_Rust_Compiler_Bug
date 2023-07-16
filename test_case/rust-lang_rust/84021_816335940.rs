rust
trait Test2 {
    fn boo<'ctx>();
}

impl Test2 for () {
    fn boo<'ctx>() where i32: 'ctx { }
}
