
warning: strict provenance disallows casting integer `usize` to pointer `*mut Client`
   --> src\tools\cargo\src/cargo\util\config\mod.rs:216:63
    |
216 |         static mut GLOBAL_JOBSERVER: *mut jobserver::Client = 0 as *mut _;
    |                                                               ^^^^^^^^^^^
    |
    = note: `#[warn(fuzzy_provenance_casts)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

warning: strict provenance disallows casting integer `usize` to pointer `*const Downloads<'_, '_>`
    --> src\tools\cargo\src/cargo\core\package.rs:1175:31
     |
1175 |             unsafe { f(Some(&*(ptr as *const Downloads<'_, '_>))) }
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
     = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

warning: strict provenance disallows casting pointer `*const Downloads<'_, '_>` to integer `usize`
    --> src\tools\cargo\src/cargo\core\package.rs:1190:19
     |
1190 |             p.set(dl as *const Downloads<'_, '_> as usize);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
     = help: use `.addr()` to obtain the address of a pointer

warning: strict provenance disallows casting pointer `*const u8` to integer `usize`
   --> src\tools\cargo\src/cargo\sources\registry\index.rs:706:31
    |
706 |             let outer_start = outer.as_ptr() as usize;
    |                               ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

warning: strict provenance disallows casting pointer `*const u8` to integer `usize`
   --> src\tools\cargo\src/cargo\sources\registry\index.rs:708:31
    |
708 |             let inner_start = inner.as_ptr() as usize;
    |                               ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer
