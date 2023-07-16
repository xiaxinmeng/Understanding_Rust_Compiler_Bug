plain
6    | |          const {
7    | |              assert!($e);
...    |
11   | |          const {
     | | _______________^
12   | ||             assert!($e, $msg);
     | ||_________^ blocks are not supported in generic constants
14   | |      };
15   | |  }
     | |__- in this expansion of `static_assert!` (#2)
     | |__- in this expansion of `static_assert!` (#2)
...
18   |  / macro_rules! static_assert_uimm_bits {
19   |  |     ($imm:ident, $bits:expr) => {
20   |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22   |  |         {
23   |  /             static_assert!(
23   |  /             static_assert!(
24   |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25   |                    concat!(
26   |                        stringify!($imm),
30   |                    )
31   |  |             )
     |  |_____________- in this macro invocation (#2)
32   |            }
32   |            }
33   |  |     };
34   |  | }
     |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1013:5
     |
     |
1013 |        static_assert_uimm_bits!(MASK, 8);
     |
     |
     = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
    --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
     |
     |
67   | /  macro_rules! simd_shuffle {
68   | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |              $x,
71   | |              $y,
72   | |              const {
     | | ___________________^
     | | ___________________^
73   | ||                 let v: [u32; _] = $idx;
75   | ||             },
     | ||_____________^ blocks are not supported in generic constants
76   | |          )
77   | |      }};
---
1022 |  |         ],
1023 |  |     )
     |  |_____- in this macro invocation
     |
     = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:426:5
    |
    |
426 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
77  | |      }};
78  | |  }
    | |__- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:444:27
    |
444 |        transmute::<i8x16, _>(simd_shuffle!(
445 |  |         zero,
445 |  |         zero,
446 |  |         a.as_i8x16(),
...    |
464 |  |         ],
465 |  |     ))
    |  |_____- in this macro invocation
    |  |_____- in this macro invocation
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:477:5
    |
    |
477 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:490:5
    |
    |
490 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:503:5
    |
    |
503 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:528:5
    |
    |
528 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:553:5
    |
    |
553 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:579:5
    |
    |
579 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:605:5
    |
    |
605 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:630:5
    |
    |
630 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
77  | |      }};
78  | |  }
    | |__- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:647:20
    |
647 |        let x: i8x16 = simd_shuffle!(
    |   ____________________-
648 |  |         a.as_i8x16(),
650 |  |         [
...    |
667 |  |         ],
668 |  |     );
668 |  |     );
    |  |_____- in this macro invocation
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:682:5
    |
    |
682 |        static_assert_uimm_bits!(IMM8, 8);
---
129 |  |         ],
130 |  |     )
    |  |_____- in this macro invocation
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:143:5
    |
    |
143 |        static_assert_uimm_bits!(MASK, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
---
156 |  |         ],
157 |  |     )
    |  |_____- in this macro invocation
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:353:5
    |
    |
353 |        static_assert_uimm_bits!(ROUNDING, 4);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:400:5
    |
    |
400 |        static_assert_uimm_bits!(ROUNDING, 4);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:465:5
    |
    |
465 |        static_assert_uimm_bits!(IMM4, 4);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
---
474 |  |         ],
475 |  |     )
    |  |_____- in this macro invocation
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:488:5
    |
    |
488 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
---
501 |  |         ],
502 |  |     )
    |  |_____- in this macro invocation
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:541:5
    |
    |
541 |        static_assert_uimm_bits!(IMM8, 8);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:740:5
    |
    |
740 |        static_assert_uimm_bits!(IMM5, 5);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:755:5
    |
    |
755 |        static_assert_uimm_bits!(IMM5, 5);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:770:5
    |
    |
770 |        static_assert_uimm_bits!(IMM5, 5);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:785:5
    |
    |
785 |        static_assert_uimm_bits!(IMM5, 5);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:802:5
    |
    |
802 |        static_assert_uimm_bits!(IMM5, 5);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:819:5
    |
    |
819 |        static_assert_uimm_bits!(IMM5, 5);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:932:5
    |
    |
932 |        static_assert_uimm_bits!(IMM1, 1);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
77  | |      }};
78  | |  }
    | |__- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:933:5
    |
933 |  /     simd_shuffle!(
934 |  |         a,
935 |  |         _mm256_undefined_ps(),
936 |  |         [[0, 1, 2, 3], [4, 5, 6, 7]][IMM1 as usize],
    |  |_____- in this macro invocation
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:953:5
    |
    |
953 |        static_assert_uimm_bits!(IMM1, 1);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |              $x,
71  | |              $y,
72  | |              const {
    | | ___________________^
    | | ___________________^
73  | ||                 let v: [u32; _] = $idx;
75  | ||             },
    | ||_____________^ blocks are not supported in generic constants
76  | |          )
77  | |      }};
77  | |      }};
78  | |  }
    | |__- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:954:5
    |
954 |        simd_shuffle!(a, _mm256_undefined_pd(), [[0, 1], [2, 3]][IMM1 as usize])
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:11:15
    |
    |
4   | /  macro_rules! static_assert {
5   | |      ($e:expr) => {
6   | |          const {
7   | |              assert!($e);
...   |
11  | |          const {
    | | _______________^
12  | ||             assert!($e, $msg);
    | ||_________^ blocks are not supported in generic constants
14  | |      };
15  | |  }
    | |__- in this expansion of `static_assert!` (#2)
    | |__- in this expansion of `static_assert!` (#2)
...
18  |  / macro_rules! static_assert_uimm_bits {
19  |  |     ($imm:ident, $bits:expr) => {
20  |  |         // `0 <= $imm` produces a warning if the immediate has an unsigned type
22  |  |         {
23  |  /             static_assert!(
23  |  /             static_assert!(
24  |                    0 <= $imm && $imm <= (1 << $bits) - 1,
25  |                    concat!(
26  |                        stringify!($imm),
30  |                    )
31  |  |             )
    |  |_____________- in this macro invocation (#2)
32  |            }
32  |            }
33  |  |     };
34  |  | }
    |  |_- in this expansion of `static_assert_uimm_bits!` (#1)
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:969:5
    |
    |
969 |        static_assert_uimm_bits!(IMM1, 1);
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:72:19
    |
    |
67  | /  macro_rules! simd_shuffle {
68  | |      ($x:expr, $y:expr, $idx:expr $(,)?) => {{
