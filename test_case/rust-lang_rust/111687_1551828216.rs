plain
   Compiling intl-memoizer v0.5.1
   Compiling intl_pluralrules v7.0.2
   Compiling fluent-langneg v0.13.0
    Checking rustc_baked_icu_data v0.0.0 (/checkout/compiler/rustc_baked_icu_data)
error[E0502]: cannot borrow `inner` as mutable because it is also borrowed as immutable
     |
     |
1996 |             unsafe { str::from_utf8_unchecked(inner.arena.alloc_slice(string.as_bytes())) };
     |                                               ----- immutable borrow occurs here
1998 |         inner.strings.push(string as *const str);
     |         ^^^^^              ------ immutable borrow later used here
     |         |
     |         mutable borrow occurs here
     |         mutable borrow occurs here

error[E0502]: cannot borrow `inner` as mutable because it is also borrowed as immutable
     |
     |
1996 |             unsafe { str::from_utf8_unchecked(inner.arena.alloc_slice(string.as_bytes())) };
     |                                               ----- immutable borrow occurs here
...
2003 |         inner.names.insert(string as *const str, name);
     |         ^^^^^              ------ immutable borrow later used here
     |         mutable borrow occurs here

    Checking fluent-bundle v0.15.2
For more information about this error, try `rustc --explain E0502`.
