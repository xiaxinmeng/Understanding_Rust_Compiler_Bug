
impl FnOnce<(Foo<'static>,)> for Func {
    type Output = String;

    extern "rust-call" fn call_once(self, args: (Foo<'static>,)) -> String {
        self.call(args)
    }
}

impl FnMut<(Foo<'static>,)> for Func {
    extern "rust-call" fn call_mut(&mut self, args: (Foo<'static>,)) -> String {
        self.call(args)
    }
}

impl Fn<(Foo<'static>,)> for Func {
    extern "rust-call" fn call(&self, (i,): (Foo<'static>,)) -> String {
        format!("{:?}", i)
    }
}
