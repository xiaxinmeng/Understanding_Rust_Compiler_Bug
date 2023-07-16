
anon>:6:15: 6:25 error: unsupported cyclic reference between types/traits detected [E0391]
<anon>:6     Self: Add<Self::Diff, Output = Self>,
                       ^~~~~~~~~~
<anon>:6:15: 6:25 help: see the detailed explanation for E0391
note: the cycle begins when computing the bounds for type parameter `Self`...
note: ...which then again requires computing the bounds for type parameter `Self`, completing the cycle.
<anon>:7:30: 7:40 error: unsupported cyclic reference between types/traits detected [E0391]
<anon>:7     Self: Sub<Self, Output = Self::Diff>,
                                      ^~~~~~~~~~
<anon>:7:30: 7:40 help: see the detailed explanation for E0391
note: the cycle begins when computing the bounds for type parameter `Self`...
note: ...which then again requires computing the bounds for type parameter `Self`, completing the cycle.
<anon>:6:15: 6:25 error: unsupported cyclic reference between types/traits detected [E0391]
<anon>:6     Self: Add<Self::Diff, Output = Self>,
                       ^~~~~~~~~~
<anon>:6:15: 6:25 help: see the detailed explanation for E0391
note: the cycle begins when computing the supertraits of `AffineSpace`...
note: ...which then again requires computing the supertraits of `AffineSpace`, completing the cycle.
error: aborting due to 3 previous errors
