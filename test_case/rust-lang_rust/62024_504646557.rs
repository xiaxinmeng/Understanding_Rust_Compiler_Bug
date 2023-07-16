
[01:47:45] [0m[0m[1m[32m   Compiling[0m miri v0.1.0 (/checkout/src/tools/miri)
[01:47:48] [0m[1m[38;5;9merror[E0277][0m[0m[1m: the `?` operator can only be applied to values that implement `std::ops::Try`[0m
[01:47:48] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/miri/src/intrinsic.rs:506:29[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m|[0m
[01:47:48] [0m[1m[38;5;12m506[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m/[0m[0m [0m[0m                            this.memory_mut()[0m
[01:47:48] [0m[1m[38;5;12m507[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m|[0m[0m [0m[0m                                .get_mut(ptr.alloc_id)?[0m
[01:47:48] [0m[1m[38;5;12m508[0m[0m [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m|[0m[0m [0m[0m                                .mark_definedness(ptr, dest.layout.size, false)?;[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m|[0m[0m                                                                                [0m[0m[1m[38;5;9m^[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m|[0m[0m                                                                                [0m[0m[1m[38;5;12m|[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m|[0m[0m                                                                                [0m[0m[1m[38;5;9mthe `?` operator cannot be applied to type `()`[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m| [0m[0m[1m[38;5;12m|________________________________________________________________________________[0m[0m[1m[38;5;12min this expansion of `desugaring of `?``[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m| [0m[0m                                                                                 [0m[0m[1m[38;5;12min this macro invocation[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m|[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mhelp[0m[0m: the trait `std::ops::Try` is not implemented for `()`[0m
[01:47:48] [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: required by `std::ops::Try::into_result`[0m
