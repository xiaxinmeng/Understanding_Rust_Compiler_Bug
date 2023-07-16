plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: any use of this value will cause an error
    --> /checkout/library/core/src/intrinsics.rs:2242:14
     |
2242 |       unsafe { write_bytes(dst, val, count) }
     |                |
     |                |
     |                "calling intrinsic `write_bytes`" needs an rfc before being allowed inside constants
     |                inside `std::intrinsics::write_bytes::<u32>` at /checkout/library/core/src/intrinsics.rs:2242:14
     |                inside `intrinsics::test_write_bytes_in_const_contexts::TEST` at library/core/tests/intrinsics.rs:45:13
    ::: library/core/tests/intrinsics.rs:42:5
     |
     |
42   | /     const TEST: [u32; 3] = {
43   | |         let mut arr = [1u32, 2, 3];
44   | |         unsafe {
45   | |             write_bytes(arr.as_mut_ptr(), 0, 2);
47   | |         arr
48   | |     };
     | |______-
     |
     |
     = note: `#[deny(const_err)]` on by default
     = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
    --> /checkout/library/core/src/intrinsics.rs:2242:14
    --> /checkout/library/core/src/intrinsics.rs:2242:14
     |
2242 |       unsafe { write_bytes(dst, val, count) }
     |                |
     |                |
     |                "calling intrinsic `write_bytes`" needs an rfc before being allowed inside constants
     |                inside `std::intrinsics::write_bytes::<u32>` at /checkout/library/core/src/intrinsics.rs:2242:14
     |                inside `TEST2` at library/core/tests/intrinsics.rs:57:13
    ::: library/core/tests/intrinsics.rs:54:5
     |
     |
54   | /     const TEST2: [u32; 3] = {
55   | |         let mut arr = [1u32, 2, 3];
56   | |         unsafe {
57   | |             write_bytes(arr.as_mut_ptr(), 1, 2);
59   | |         arr
60   | |     };
     | |______-
     |
