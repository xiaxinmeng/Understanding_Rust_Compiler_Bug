
pietro@backdoor: ~/r/github/rust-lang/rust ((9fdc5f228e9...))$ rg '#!?\[.*\(bootstrap' -t rust
library/core/src/prelude/v1.rs
78:#[cfg(not(bootstrap))]

library/core/tests/mem.rs
80:#[cfg(not(bootstrap))] // stage 0 doesn't have the fix yet, so the test fails

library/core/src/macros/mod.rs
1517:    #[cfg(not(bootstrap))]

[ignoring stdarch]

library/alloc/src/alloc.rs
408:    #[cfg(bootstrap)]

library/std/src/prelude/v1.rs
62:#[cfg(not(bootstrap))]
pietro@backdoor: ~/r/github/rust-lang/rust ((9fdc5f228e9...))$ 
