
error[E0283]: type annotations required: cannot resolve `_: kernel::hil::symmetric_encryption::AES128<'_>`
   --> src/aes.rs:162:22
    |
162 |             sublen % AES128::BLOCK_SIZE == 0 && {
    |                      ^^^^^^^^^^^^^^^^^^
    |
    = note: required by `kernel::hil::symmetric_encryption::AES128::BLOCK_SIZE`

error: aborting due to previous error
