plain
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0521]: borrowed data escapes outside of associated function
    |
153 |         &self,
    |         -----
    |         |
    |         |
    |         `self` is a reference that is only valid in the associated function body
    |         let's call the lifetime of this reference `'1`
192 |                 Allocator::deallocate(&self, ptr, old_layout);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |                 |
    |                 |
    |                 `self` escapes the associated function body here
    |                 argument requires that `'1` must outlive `'static`

error[E0521]: borrowed data escapes outside of associated function
    |
246 |         &self,
    |         -----
    |         |
    |         |
    |         `self` is a reference that is only valid in the associated function body
    |         let's call the lifetime of this reference `'1`
279 |                 let new_ptr = Allocator::allocate(&self, new_layout)?;
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |                               |
    |                               |
    |                               `self` escapes the associated function body here
    |                               argument requires that `'1` must outlive `'static`
For more information about this error, try `rustc --explain E0521`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:01:46
