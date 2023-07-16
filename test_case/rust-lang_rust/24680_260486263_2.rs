
#![feature(fn_traits)]
#![feature(unboxed_closures)]

// Foo and IntoBox unchanged

struct Func;

impl<'a> FnOnce<(Foo<'a>,)> for Func {
    type Output = String;

    extern "rust-call" fn call_once(self, args: (Foo<'a>,)) -> String {
        self.call(args)
    }
}

impl<'a> FnMut<(Foo<'a>,)> for Func {
    extern "rust-call" fn call_mut(&mut self, args: (Foo<'a>,)) -> String {
        self.call(args)
    }
}

impl<'a> Fn<(Foo<'a>,)> for Func {
    extern "rust-call" fn call(&self, (i,): (Foo<'a>,)) -> String {
        format!("{:?}", i)
    }
}

fn main() {
    let x = IntoBox::into_box(Func);
}
