 rust
#![feature(no_std,lang_items,fundamental)]
#![no_std]
#![crate_type="lib"]
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
#[lang="no_copy_bound"]
pub struct NoCopy;

pub extern "stdcall" fn sup(_: isize) -> isize {
    0
}
