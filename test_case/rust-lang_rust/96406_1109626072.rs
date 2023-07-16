rust
#![feature(type_alias_impl_trait)]

use core::future::Future;
use core::mem::MaybeUninit;
use core::pin::Pin;
struct Foo;

fn foo(f: &'static mut impl Future) -> &'static mut impl Future { f }
fn task(arg: Foo) -> &'static mut impl Future {
    async fn task_inner(arg: Foo) {
        // code
    }

    type Fut = impl Future;
    static mut FUT: MaybeUninit<Fut> = MaybeUninit::uninit();
    foo(unsafe { FUT.write(task_inner(arg)) })
}
