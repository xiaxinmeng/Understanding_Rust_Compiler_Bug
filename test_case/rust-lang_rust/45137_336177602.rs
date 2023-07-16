
[00:08:12] [1m[31merror(B[m[1m: recursion limit reached while expanding the macro `stringify`(B[m
[00:08:12]    (B[m[1m[34m--> (B[m/checkout/src/librustc/ty/maps/plumbing.rs:563:26(B[m
[00:08:12]     (B[m[1m[34m|(B[m
[00:08:12] [1m[34m563(B[m (B[m[1m[34m| (B[m  (B[m                         stringify!($name), key);(B[m
[00:08:12]     (B[m[1m[34m| (B[m                           (B[m[1m[31m^^^^^^^^^^^^^^^^^(B[m
[00:08:12]     (B[m[1m[34m| (B[m
[00:08:12]    (B[m[1m[34m::: (B[m/checkout/src/librustc/ty/maps/mod.rs(B[m
[00:08:12]     (B[m[1m[34m|(B[m
[00:08:12] [1m[34m78(B[m  (B[m[1m[34m| (B[m[1m[34m/(B[m (B[mdefine_maps! { <'tcx>(B[m
[00:08:12] [1m[34m79(B[m  (B[m[1m[34m| (B[m[1m[34m|(B[m (B[m    /// Records the type of every item.(B[m
[00:08:12] [1m[34m80(B[m  (B[m[1m[34m| (B[m[1m[34m|(B[m (B[m    [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,(B[m
[00:08:12] [1m[34m81(B[m  (B[m[1m[34m| (B[m[1m[34m|(B[m
[00:08:12] [1m[34m...(B[m   (B[m[1m[34m|(B[m
[00:08:12] [1m[34m341(B[m (B[m[1m[34m| (B[m[1m[34m|(B[m (B[m    [] fn has_clone_closures: HasCloneClosures(CrateNum) -> bool,(B[m
[00:08:12] [1m[34m342(B[m (B[m[1m[34m| (B[m[1m[34m|(B[m (B[m}(B[m
[00:08:12]     (B[m[1m[34m| (B[m[1m[34m|_-(B[m (B[m[1m[34min this macro invocation(B[m
[00:08:12]     (B[m[1m[34m|(B[m
[00:08:12]     (B[m[1m[34m= (B[m[1mhelp(B[m: consider adding a `#![recursion_limit="512"]` attribute to your crate(B[m
[00:08:12] 
[00:08:12] [m[m[31m[1merror:[m Could not compile `rustc`.
