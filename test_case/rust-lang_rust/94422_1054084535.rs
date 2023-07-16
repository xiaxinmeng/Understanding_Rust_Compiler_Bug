
impl<'h,S: System<'h,Host> + Any> typemap::Key for S {
        type Value = SystemData<'h,S>;
}
