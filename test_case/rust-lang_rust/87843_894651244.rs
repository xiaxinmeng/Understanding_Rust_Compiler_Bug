plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
  |
7 | use std::assert_matches::assert_matches;
  |     ^^^ use of undeclared crate or module `std`

error: cannot find macro `assert_matches` in this scope
   --> library/std/src/collections/hash/map/tests.rs:831:5
831 |     assert_matches!(
    |     ^^^^^^^^^^^^^^
    |
    = note: consider importing one of these items:
    = note: consider importing one of these items:
            core::assert_matches::assert_matches
            crate::assert_matches::assert_matches

error: cannot find macro `assert_matches` in this scope
   --> library/std/src/collections/hash/map/tests.rs:825:5
825 |     assert_matches!(
    |     ^^^^^^^^^^^^^^
    |
    = note: consider importing one of these items:
    = note: consider importing one of these items:
            core::assert_matches::assert_matches
            crate::assert_matches::assert_matches

error: unused import: `realstd::collections::TryReserveErrorKind::*`
 --> library/std/src/collections/hash/map/tests.rs:6:5
  |
6 | use realstd::collections::TryReserveErrorKind::*;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:719:17
    |
    |
718 |                 empty_string.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
    |                 ----------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
719 |                 CapacityOverflow,
    |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:726:17
    |
    |
725 |                 empty_string.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
    |                 --------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
726 |                 CapacityOverflow,
    |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:733:17
    |
    |
732 |                 empty_string.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
    |                 ----------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
733 |                 AllocError { .. },
    |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:740:17
    |
    |
739 |                 empty_string.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
    |                 --------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
740 |                 AllocError { .. },
    |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:759:17
    |
    |
758 |                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
    |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
759 |                 CapacityOverflow,
    |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:765:17
    |
    |
764 |                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
    |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
765 |                 AllocError { .. },
    |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:772:13
    |
    |
771 |             ten_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
    |             ------------------------------------------------------ this expression has type `Result<(), TryReserveErrorKind>`
772 |             CapacityOverflow,
    |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:805:17
    |
    |
804 |                 empty_string.try_reserve_exact(MAX_CAP + 1).map_err(|e| e.kind()),
    |                 ----------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
805 |                 CapacityOverflow,
    |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:811:17
    |
    |
810 |                 empty_string.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
    |                 --------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
811 |                 CapacityOverflow,
    |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:817:17
    |
    |
816 |                 empty_string.try_reserve_exact(MAX_CAP + 1).map_err(|e| e.kind()),
    |                 ----------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
817 |                 AllocError { .. },
    |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:823:17
    |
    |
822 |                 empty_string.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
    |                 --------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
823 |                 AllocError { .. },
    |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:845:17
    |
    |
844 |                 ten_bytes.try_reserve_exact(MAX_CAP - 9).map_err(|e| e.kind()),
    |                 -------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
845 |                 CapacityOverflow,
    |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:851:17
    |
    |
850 |                 ten_bytes.try_reserve_exact(MAX_CAP - 9).map_err(|e| e.kind()),
    |                 -------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
851 |                 AllocError { .. },
    |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/tests/string.rs:857:13
    |
    |
856 |             ten_bytes.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
    |             ------------------------------------------------------------ this expression has type `Result<(), TryReserveErrorKind>`
857 |             CapacityOverflow,
    |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
    |
    = note: expected enum `Result<(), TryReserveErrorKind>`
               found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1494:17
     |
     |
1493 |                 empty_bytes.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
     |                 ---------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1494 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1501:17
     |
     |
1500 |                 empty_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1501 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1508:17
     |
     |
1507 |                 empty_bytes.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
     |                 ---------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1508 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1515:17
     |
     |
1514 |                 empty_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1515 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1534:17
     |
     |
1533 |                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1534 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1540:17
     |
     |
1539 |                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1540 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1547:13
     |
     |
1546 |             ten_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
     |             ------------------------------------------------------ this expression has type `Result<(), TryReserveErrorKind>`
1547 |             CapacityOverflow,
     |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1567:17
     |
     |
1566 |                 ten_u32s.try_reserve(MAX_CAP / 4 - 9).map_err(|e| e.kind()),
     |                 ----------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1567 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1573:17
     |
     |
1572 |                 ten_u32s.try_reserve(MAX_CAP / 4 - 9).map_err(|e| e.kind()),
     |                 ----------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1573 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1580:13
     |
     |
1579 |             ten_u32s.try_reserve(MAX_USIZE - 20).map_err(|e| e.kind()),
     |             ---------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1580 |             CapacityOverflow,
     |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1613:17
     |
     |
1612 |                 empty_bytes.try_reserve_exact(MAX_CAP + 1).map_err(|e| e.kind()),
     |                 ---------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1613 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1619:17
     |
     |
1618 |                 empty_bytes.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
     |                 -------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1619 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1625:17
     |
     |
1624 |                 empty_bytes.try_reserve_exact(MAX_CAP + 1).map_err(|e| e.kind()),
     |                 ---------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1625 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1631:17
     |
     |
1630 |                 empty_bytes.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
     |                 -------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1631 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1653:17
     |
     |
1652 |                 ten_bytes.try_reserve_exact(MAX_CAP - 9).map_err(|e| e.kind()),
     |                 -------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1653 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1659:17
     |
     |
1658 |                 ten_bytes.try_reserve_exact(MAX_CAP - 9).map_err(|e| e.kind()),
     |                 -------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1659 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1665:13
     |
     |
1664 |             ten_bytes.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
     |             ------------------------------------------------------------ this expression has type `Result<(), TryReserveErrorKind>`
1665 |             CapacityOverflow,
     |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1686:17
     |
     |
1685 |                 ten_u32s.try_reserve_exact(MAX_CAP / 4 - 9).map_err(|e| e.kind()),
     |                 ----------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1686 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1692:17
     |
     |
1691 |                 ten_u32s.try_reserve_exact(MAX_CAP / 4 - 9).map_err(|e| e.kind()),
     |                 ----------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1692 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec.rs:1698:13
     |
     |
1697 |             ten_u32s.try_reserve_exact(MAX_USIZE - 20).map_err(|e| e.kind()),
     |             ---------------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1698 |             CapacityOverflow,
     |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1187:17
     |
     |
1186 |                 empty_bytes.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
     |                 ---------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1187 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1194:17
     |
     |
1193 |                 empty_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1194 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1204:17
     |
     |
1203 |                 empty_bytes.try_reserve(MAX_CAP).map_err(|e| e.kind()),
     |                 ------------------------------------------------------ this expression has type `Result<(), TryReserveErrorKind>`
1204 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1223:17
     |
     |
1222 |                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1223 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1229:17
     |
     |
1228 |                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
     |                 -------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1229 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1236:13
     |
     |
1235 |             ten_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
     |             ------------------------------------------------------ this expression has type `Result<(), TryReserveErrorKind>`
1236 |             CapacityOverflow,
     |             ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1256:17
     |
     |
1255 |                 ten_u32s.try_reserve(MAX_CAP / 4 - 9).map_err(|e| e.kind()),
     |                 ----------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1256 |                 CapacityOverflow,
     |                 ^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1262:17
     |
     |
1261 |                 ten_u32s.try_reserve(MAX_CAP / 4 - 9).map_err(|e| e.kind()),
     |                 ----------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1262 |                 AllocError { .. },
     |                 ^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `TryReserveErrorKind`
     |
     = note: expected enum `Result<(), TryReserveErrorKind>`
                found enum `TryReserveErrorKind`
error[E0308]: mismatched types
    --> library/alloc/tests/vec_deque.rs:1269:13
     |
     |
1268 |             ten_u32s.try_reserve(MAX_USIZE - 20).map_err(|e| e.kind()),
     |             ---------------------------------------------------------- this expression has type `Result<(), TryReserveErrorKind>`
1269 |             CapacityOverflow,
