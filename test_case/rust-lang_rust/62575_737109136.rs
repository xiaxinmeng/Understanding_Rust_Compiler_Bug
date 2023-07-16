rust
#![feature(rustc_attrs, type_alias_impl_trait)]

use core::future::Future;

#[rustc_layout(debug)]
type T = impl Future;

fn f() -> T {
    async {}
}
