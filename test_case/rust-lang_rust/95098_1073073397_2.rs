console
error[E0277]: the trait bound `Vec<u8>: From<&[u8; 3]>` is not satisfied
   --> src/main.rs:2:36
    |
2   |     let _ = std::ffi::CString::new(b"+\xff+");
    |             ---------------------- ^^^^^^^^^ the trait `From<&[u8; 3]>` is not implemented for `Vec<u8>`
    |             |
    |             required by a bound introduced by this call
    |
    = help: the following implementations were found:
              <Vec<u8> as From<&str>>
              <Vec<u8> as From<CString>>
              <Vec<u8> as From<String>>
              <Vec<T, A> as From<Box<[T], A>>>
            and 6 others
    = note: required because of the requirements on the impl of `Into<Vec<u8>>` for `&[u8; 3]`
note: required by a bound in `CString::new`
