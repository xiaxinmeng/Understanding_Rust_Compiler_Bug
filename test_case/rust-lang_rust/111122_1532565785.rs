plain
    Checking miniz_oxide v0.6.2
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.30.1
    Checking addr2line v0.19.0
error[E0277]: a value of type `&Path` cannot be built from an iterator over elements of type `u16`
    --> library/std/src/sys/windows/fs.rs:545:87
     |
545  |                 let user = super::args::to_user_path(subst.iter().copied().chain([0]).collect())?;
     |                                                                                       ^^^^^^^ value of type `&Path` cannot be built from `std::iter::Iterator<Item=u16>`
     = help: the trait `core::iter::FromIterator<u16>` is not implemented for `&Path`
note: the method call chain might not have had the expected associated types
    --> library/std/src/sys/windows/fs.rs:545:67
     |
     |
538  |             let subst = slice::from_raw_parts_mut(subst_ptr, subst_len as usize);
     |                         -------------------------------------------------------- this expression has type `&mut [u16]`
...
545  |                 let user = super::args::to_user_path(subst.iter().copied().chain([0]).collect())?;
     |                                                            ------ ^^^^^^^^ ---------- `Iterator::Item` remains `u16` here
     |                                                            |      |
     |                                                            |      `Iterator::Item` changed to `u16` here
     |                                                            `Iterator::Item` is `&u16` here
    --> /checkout/library/core/src/iter/traits/iterator.rs:1891:19
     |
     |
1891 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::collect`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `std` (lib) due to previous error
Build completed unsuccessfully in 0:00:12
