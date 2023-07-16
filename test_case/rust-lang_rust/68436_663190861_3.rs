
LL fn bar() { foo::<1>(); }
                    ^ overflow during evaluation of `1_usize - 2_usize`
note: required due to
LL fn foo<const C: usize>() where (C - 2) {
                                   ^^^^^
