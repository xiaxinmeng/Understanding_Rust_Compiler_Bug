
LL fn bar() { foo::<1>(); }
                    ^ overflow during evaluation of `1_usize - 2_usize`
note: required due to the constant used in the body of `foo`
LL let x = [u8; C - 2];
                ^^^^^
