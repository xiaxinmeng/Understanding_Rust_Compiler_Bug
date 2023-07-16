
error: symbol `_ZN100_$LT$core..iter..adapters..fuse..Fuse$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h8a5af79663957c6eE` is already defined
   --> src/libcore/iter/adapters/fuse.rs:93:5
    |
93  | /     fn try_fold<Acc, Fold, R>(&mut self, acc: Acc, fold: Fold) -> R
94  | |     where
95  | |         Self: Sized,
96  | |         Fold: FnMut(Acc, Self::Item) -> R,
...   |
99  | |         FuseImpl::try_fold(self, acc, fold)
100 | |     }
    | |_____^

error: aborting due to previous error
