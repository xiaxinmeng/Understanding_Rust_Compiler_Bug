 shell
$ RUST_BACKTRACE=1 rustc - <<< 'fn main() { struct P<A, B>; type Q<C> = P<C::A, C::B>; }'
<anon>:1:43: 1:47 error: associated type `A` not found for `C` [E0220]
<anon>:1 fn main() { struct P<A, B>; type Q<C> = P<C::A, C::B>; }
                                                   ^~~~
<anon>:1:49: 1:53 error: associated type `B` not found for `C` [E0220]
<anon>:1 fn main() { struct P<A, B>; type Q<C> = P<C::A, C::B>; }
                                                         ^~~~
error: aborting due to 2 previous errors
