 rust

struct MemberFn<'a, T, R, A0> where T : 'a {
    fun: fn (&T, A0) -> R,
    obj: &'a T
}

impl<'a, T, R, A0> MemberFn<'a, T, R, A0> where T : 'a {
    fn new(o: &'a T, f: fn (&T, A0) -> R) -> MemberFn<'a, T, R, A0> {
        MemberFn {
            fun: f,
            obj: o
        }
    }
}

impl<'a, T, R, A0, Args> Fn<Args, R> for MemberFn<'a, T, R, A0> where Args : Tuple1<A0> {
   fn call(&self, args: Args) -> R {
        (self.fun)(self.obj, args.val0())
    }
}

fn main() {
    println!("Hello, world!")
}
