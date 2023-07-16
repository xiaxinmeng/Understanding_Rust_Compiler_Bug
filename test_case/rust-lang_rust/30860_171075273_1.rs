
<anon>:44:14: 44:26 error: type mismatch: the type `[closure@<anon>:40:19: 40:62]` implements the trait `for<'r> core::ops::Fn<(&'r str,)>`, but the trait `for<'r> core::ops::Fn<(<Str as TypeWithLifetime<'r>>::Type,)>` is required (expected associated type, found &-ptr) [E0281]
<anon>:44     consumer.accept("hi");
                       ^~~~~~~~~~~~
<anon>:44:14: 44:26 help: see the detailed explanation for E0281
<anon>:44:14: 44:26 error: type mismatch: the type `[closure@<anon>:40:19: 40:62]` implements the trait `for<'r> core::ops::FnOnce<(&'r str,)>`, but the trait `for<'r> core::ops::FnOnce<(<Str as TypeWithLifetime<'r>>::Type,)>` is required (expected associated type, found &-ptr) [E0281]
<anon>:44     consumer.accept("hi");
                       ^~~~~~~~~~~~
<anon>:44:14: 44:26 help: see the detailed explanation for E0281
error: aborting due to 2 previous errors
