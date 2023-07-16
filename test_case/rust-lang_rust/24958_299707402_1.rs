rust
#![feature(no_core,lang_items,fundamental)]
#![no_core]
#[lang="sized"]
#[fundamental]
pub trait Sized {}
#[lang="sync"]
pub unsafe trait Sync {}
#[lang="phantom_data"]
pub struct PhantomData<T:?Sized>;
#[lang="send"]
pub unsafe trait Send {}
#[lang="copy"]
pub trait Copy {}

pub extern "stdcall" fn sup(_: isize) -> isize {
    0
}
