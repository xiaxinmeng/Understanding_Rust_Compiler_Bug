plain
.................................................................................................... 9300/11524
.................................................................................................... 9400/11524
..........................................................................i.....i................... 9500/11524
.................................................................................................... 9600/11524
............iiiiiii..iiiiii.i....................................................................... 9700/11524
.................................................................................................... 9900/11524
.................................................................................................... 10000/11524
.................................................................................................... 10100/11524
.................................................................................................... 10200/11524
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.072 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.452 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1300/2876
.................................................................................................... 1400/2876
.................................................................................................... 1500/2876
.................................................................................................... 1600/2876
.......................................................................F.FF....FF.F.F.FF.FF..FFFFF.F 1700/2876
...FFFF.FF.F..FFF..F.FF.FFF.FFF..................................................................... 1800/2876
.................................................................................................... 2000/2876
.................................................................................................... 2100/2876
.................................................................................................... 2200/2876
.................................................................................................... 2300/2876
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::checked_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU128::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU128::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU128::new(u128::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::checked_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:28
  |
6 | let max = NonZeroU128::new($ Int::MAX).unwrap();
  |                            ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU128::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU128::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::checked_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:33
  |
6 | let half_max = NonZeroU128::new($ Int::MAX / 2).unwrap();
  |                                 ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU128::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU128::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::saturating_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU128::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU128::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU128::new(u128::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::saturating_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:28
  |
6 | let max = NonZeroU128::new($ Int::MAX).unwrap();
  |                            ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU128::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU128::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::saturating_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:28
  |
6 | let max = NonZeroU128::new($ Int::MAX).unwrap();
  |                            ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU128::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
3 | use core::num::NonZeroU128;
  |
3 | use std::num::NonZeroU128;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU128`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU128::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU128;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::checked_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU16::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU16::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU16::new(u16::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::checked_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:27
  |
6 | let max = NonZeroU16::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU16::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU16::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::checked_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:32
  |
6 | let half_max = NonZeroU16::new($ Int::MAX / 2).unwrap();
  |                                ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU16::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU16::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::saturating_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:27
  |
6 | let max = NonZeroU16::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU16::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU16::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::saturating_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU16::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU16::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU16::new(u16::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::saturating_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:27
  |
6 | let max = NonZeroU16::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU16::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
3 | use core::num::NonZeroU16;
  |
3 | use std::num::NonZeroU16;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU16`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU16::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU16;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::checked_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU32::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU32::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU32::new(u32::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::checked_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:32
  |
6 | let half_max = NonZeroU32::new($ Int::MAX / 2).unwrap();
  |                                ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU32::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU32::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::checked_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:27
  |
6 | let max = NonZeroU32::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU32::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU32::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::saturating_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU32::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU32::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU32::new(u32::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::saturating_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:27
  |
6 | let max = NonZeroU32::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU32::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU32::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::saturating_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:27
  |
6 | let max = NonZeroU32::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU32::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
3 | use core::num::NonZeroU32;
  |
3 | use std::num::NonZeroU32;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU32::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU32;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::checked_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU64::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU64::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU64::new(u64::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::checked_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:27
  |
6 | let max = NonZeroU64::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU64::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU64::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::checked_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:32
  |
6 | let half_max = NonZeroU64::new($ Int::MAX / 2).unwrap();
  |                                ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU64::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU64::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::saturating_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:27
  |
6 | let max = NonZeroU64::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU64::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU64::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::saturating_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:27
  |
6 | let max = NonZeroU64::new($ Int::MAX).unwrap();
  |                           ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU64::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU64::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::saturating_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU64::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU64::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
3 | use core::num::NonZeroU64;
  |
3 | use std::num::NonZeroU64;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU64`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU64::new(u64::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU64;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::checked_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU8::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU8::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU8::new(u8::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::checked_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:26
  |
6 | let max = NonZeroU8::new($ Int::MAX).unwrap();
  |                          ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU8::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU8::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::checked_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:31
  |
6 | let half_max = NonZeroU8::new($ Int::MAX / 2).unwrap();
  |                               ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU8::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU8::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::saturating_add (line 347) stdout ----
error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:348:11
  |
4 | let one = NonZeroU8::new(1).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroU8::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroU8::new(u8::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::saturating_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:26
  |
6 | let max = NonZeroU8::new($ Int::MAX).unwrap();
  |                          ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroU8::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroU8::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::saturating_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:26
  |
6 | let max = NonZeroU8::new($ Int::MAX).unwrap();
  |                          ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroU8::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
3 | use core::num::NonZeroU8;
  |
3 | use std::num::NonZeroU8;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroU8::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroU8;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroUsize::checked_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:29
  |
6 | let max = NonZeroUsize::new($ Int::MAX).unwrap();
  |                             ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroUsize::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
3 | use core::num::NonZeroUsize;
  |
3 | use std::num::NonZeroUsize;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroUsize::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroUsize::checked_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:34
  |
6 | let half_max = NonZeroUsize::new($ Int::MAX / 2).unwrap();
  |                                  ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroUsize::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
3 | use core::num::NonZeroUsize;
  |
3 | use std::num::NonZeroUsize;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroUsize::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
---

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroUsize::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
3 | use core::num::NonZeroUsize;
  |
3 | use std::num::NonZeroUsize;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroUsize::new(usize::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
---

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:349:11
  |
5 | let two = NonZeroUsize::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
3 | use core::num::NonZeroUsize;
  |
3 | use std::num::NonZeroUsize;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:350:11
  |
6 | let max = NonZeroUsize::new(usize::MAX).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroUsize::saturating_mul (line 410) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:413:29
  |
6 | let max = NonZeroUsize::new($ Int::MAX).unwrap();
  |                             ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:411:11
  |
  |
4 | let two = NonZeroUsize::new(2).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
3 | use core::num::NonZeroUsize;
  |
3 | use std::num::NonZeroUsize;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:412:12
  |
5 | let four = NonZeroUsize::new(4).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
---
---- src/num/nonzero.rs - num::nonzero::NonZeroUsize::saturating_pow (line 471) stdout ----
error: expected expression, found `$`
 --> src/num/nonzero.rs:474:29
  |
6 | let max = NonZeroUsize::new($ Int::MAX).unwrap();
  |                             ^ expected expression
error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:472:13
  |
  |
4 | let three = NonZeroUsize::new(3).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
3 | use core::num::NonZeroUsize;
  |
3 | use std::num::NonZeroUsize;
  |

error[E0433]: failed to resolve: use of undeclared type `NonZeroUsize`
 --> src/num/nonzero.rs:473:20
  |
5 | let twenty_seven = NonZeroUsize::new(27).unwrap();
  |
help: consider importing one of these items
  |
3 | use core::num::NonZeroUsize;
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:05
