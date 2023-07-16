 rust
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![crate_type="rlib"]

pub trait Trait {}
pub struct Button;
pub struct Callback;

#[inline(never)]
fn other<'a, 'b>(args: (&'a Button, &'b mut Trait)) {
    let _ = args;
}

impl<'a, 'b> ::std::ops::FnOnce<(&'a Button, &'b mut Trait)> for Callback
{
    type Output = ();

    extern "rust-call" fn call_once(self, args: (&'a Button, &'b mut Trait)) {
        // self.0.call(args)
        other(args)
    }
}
