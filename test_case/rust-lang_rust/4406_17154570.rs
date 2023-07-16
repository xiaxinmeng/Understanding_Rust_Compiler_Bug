
% cat /tmp/bug2.rs
trait Op<T> { fn call(self, T, T) -> T; }

impl<T> Op<T> for int   { fn call(self,  x:T, _y:T) -> T { x } }
impl<T> Op<T> for float { fn call(self, _x:T,  y:T) -> T { y } }

fn main() {
    let a : int   = 3i;
    let b : float = 4f;
    io::println(fmt!("a: %d", a.call(4,5)));
    io::println(fmt!("b: %d", b.call(4,5)));
}
% ./x86_64-apple-darwin/stage2/bin/rustc /tmp/bug2.rs
Assertion failed: (S->getType()->isPtrOrPtrVectorTy() && "Invalid cast"), function CreatePointerCast, file /Users/fklock/Dev/Mozilla/rust.git/src/llvm/lib/IR/Instructions.cpp, line 2394.
Abort trap: 6
