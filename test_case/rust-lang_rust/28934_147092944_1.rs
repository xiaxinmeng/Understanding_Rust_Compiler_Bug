
<anon>:12:20: 12:64 error: type mismatch: the type `fn(&mut Parser<'_, '_>) -> core::result::Result<(), ()> {Parser<'i, 't>::expect_exhausted}` implements the trait `for<'r> core::ops::FnOnce<(&'r mut Parser<'_, '_>,)>`, but the trait `for<'r, 'tt> core::ops::FnOnce<(&'r mut Parser<'_, 'tt>,)>` is required (expected concrete lifetime, found bound lifetime parameter 'tt) [E0281]
<anon>:12     Parser(&x, &x).parse_nested_block(Parser::expect_exhausted).unwrap();
                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:12:20: 12:64 error: type mismatch resolving `for<'r, 'tt> <fn(&mut Parser<'_, '_>) -> core::result::Result<(), ()> {Parser<'i, 't>::expect_exhausted} as core::ops::FnOnce<(&'r mut Parser<'_, 'tt>,)>>::Output == _`:
 expected bound lifetime parameter 'tt,
    found concrete lifetime [E0271]
<anon>:12     Parser(&x, &x).parse_nested_block(Parser::expect_exhausted).unwrap();
                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

<anon>:12:20: 12:64 help: see the detailed explanation for E0271
error: aborting due to 2 previous errors
