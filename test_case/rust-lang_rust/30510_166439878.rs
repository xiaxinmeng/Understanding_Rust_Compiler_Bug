
<anon>:11:1: 11:42 error: cross-crate traits with a default impl, like `std::panic::RecoverSafe`, can only be implemented for a struct/enum type, not `&'a MyWrapper` [E0321]
<anon>:11 impl<'a> RecoverSafe for &'a MyWrapper {}
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:11:1: 11:42 help: see the detailed explanation for E0321
<anon>:11:1: 11:42 error: conflicting implementations of trait `std::panic::RecoverSafe` for type `&MyWrapper`: [E0119]
<anon>:11 impl<'a> RecoverSafe for &'a MyWrapper {}
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:11:1: 11:42 help: see the detailed explanation for E0119
note: conflicting implementation in crate `std`
error: aborting due to 2 previous errors
playpen: application terminated with error code 101
