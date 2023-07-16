plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: any use of this value will cause an error
   --> library/core/tests/cmp.rs:228:23
    |
228 | const _: () = assert!(S(0) != S(1));
    |                       |
    |                       |
    |                       "calling extern function `std::cmp::PartialEq::ne`" needs an rfc before being allowed inside constants
    |
    = note: `#[deny(const_err)]` on by default
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
   --> library/core/tests/cmp.rs:230:23
   --> library/core/tests/cmp.rs:230:23
    |
230 | const _: () = assert!(S(1) <= S(1));
    |                       |
    |                       |
    |                       "calling extern function `std::cmp::PartialOrd::le`" needs an rfc before being allowed inside constants
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
   --> library/core/tests/cmp.rs:231:23
    |
231 | const _: () = assert!(S(1) >= S(1));
    |                       |
    |                       |
    |                       "calling extern function `std::cmp::PartialOrd::ge`" needs an rfc before being allowed inside constants
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
   --> library/core/tests/cmp.rs:232:23
    |
232 | const _: () = assert!(S(0) <  S(1));
    |                       |
    |                       |
    |                       "calling extern function `std::cmp::PartialOrd::lt`" needs an rfc before being allowed inside constants
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
   --> library/core/tests/cmp.rs:233:23
    |
233 | const _: () = assert!(S(1) >  S(0));
    |                       |
    |                       |
    |                       "calling extern function `std::cmp::PartialOrd::gt`" needs an rfc before being allowed inside constants
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not compile `core` due to 5 previous errors
