plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling object v0.26.1
   Compiling addr2line v0.16.0
error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
495 |             Box::into_raw(Box::new(contents))
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid

error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
    |
508 |             unsafe { Box::into_raw(Box::new(self.0)) }
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid

error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
    |
508 |             unsafe { Box::into_raw(Box::new(self.0)) }
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid

error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
    |
570 |             unsafe { Box::into_raw(data) }
    |                      ^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid

error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
    |
570 |             unsafe { Box::into_raw(data) }
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid

error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
    |
657 |             unsafe { Box::into_raw(mem::replace(&mut self.0, Box::new(()))) }
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid

error[E0133]: performing unsizing coercion is unsafe and requires unsafe block
    |
    |
657 |             unsafe { Box::into_raw(mem::replace(&mut self.0, Box::new(()))) }
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ performing unsizing coercion
    |
    = note: unsizing coercion performed on a type that did not implement `CoerceUnsized` will cause undefined behavior if data is invalid
For more information about this error, try `rustc --explain E0133`.
error: could not compile `std` due to 7 previous errors
Build completed unsuccessfully in 0:05:09
