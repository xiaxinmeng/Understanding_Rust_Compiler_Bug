rust
#![feature(const_generics)]

struct Test(*const usize);

type PassArg = ();

unsafe extern "C" fn pass(args: PassArg) {
    println!("Hello, world!");
}

impl Test {
    pub fn call_me<Args: Sized, const IDX: usize, const FN: unsafe extern "C" fn(Args)>(&self) {
        self.0 = Self::trampiline::<Args, IDX, FN> as _
    }
    
    unsafe extern "C" fn trampiline<
        Args: Sized,
        const IDX: usize,
        const FN: unsafe extern "C" fn(Args),
    >(
        args: Args,
    ) {
        FN(args)
    }
}


fn main() {
    let x = Test();
    x.call_me::<PassArg, 30, pass>()
}
