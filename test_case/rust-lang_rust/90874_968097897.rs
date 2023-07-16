plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0405]: cannot find trait `Add` in this scope
  --> library/core/src/ops/range.rs:99:11
   |
99 | impl<Idx: Add<Idx> + Copy> Range<Idx> {
   |
help: consider importing this trait
   |
1  | use crate::ops::Add;
1  | use crate::ops::Add;
   |

error[E0405]: cannot find trait `Sub` in this scope
    |
    |
118 | impl<Idx: Sub<Idx> + Copy> Range<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Sub;
1   | use crate::ops::Sub;
    |

error[E0405]: cannot find trait `Add` in this scope
   --> library/core/src/ops/range.rs:240:11
    |
240 | impl<Idx: Add<Idx> + Copy> RangeFrom<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Add;
1   | use crate::ops::Add;
    |

error[E0405]: cannot find trait `Sub` in this scope
    |
    |
259 | impl<Idx: Sub<Idx> + Copy> RangeFrom<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Sub;
1   | use crate::ops::Sub;
    |

error[E0405]: cannot find trait `Add` in this scope
   --> library/core/src/ops/range.rs:359:11
    |
359 | impl<Idx: Add<Idx> + Copy> RangeTo<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Add;
1   | use crate::ops::Add;
    |

error[E0405]: cannot find trait `Sub` in this scope
    |
    |
378 | impl<Idx: Sub<Idx> + Copy> RangeTo<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Sub;
1   | use crate::ops::Sub;
    |

error[E0405]: cannot find trait `Add` in this scope
   --> library/core/src/ops/range.rs:473:11
    |
473 | impl<Idx: Add<Idx> + Copy> RangeInclusive<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Add;
1   | use crate::ops::Add;
    |

error[E0405]: cannot find trait `Sub` in this scope
    |
    |
496 | impl<Idx: Sub<Idx> + Copy> RangeInclusive<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Sub;
1   | use crate::ops::Sub;
    |

error[E0405]: cannot find trait `Add` in this scope
   --> library/core/src/ops/range.rs:761:11
    |
761 | impl<Idx: Add<Idx> + Copy> RangeToInclusive<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Add;
1   | use crate::ops::Add;
    |

error[E0405]: cannot find trait `Sub` in this scope
    |
    |
780 | impl<Idx: Sub<Idx> + Copy> RangeToInclusive<Idx> {
    |
help: consider importing this trait
    |
1   | use crate::ops::Sub;
