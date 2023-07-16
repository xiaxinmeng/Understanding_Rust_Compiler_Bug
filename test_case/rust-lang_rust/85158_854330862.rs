plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:148 ~ rustc_demangle[f982]::demangle), const_param_did: None }) (end of phase Optimization) at bb16[17]:
bad index (isize != usize)
   |
   |
86 |             Some(DemangleStyle::Legacy(d))
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:148 ~ rustc_demangle[f982]::demangle), const_param_did: None }) (end of phase Optimization) at bb20[17]:
bad index (isize != usize)
   |
   |
91 |                 Some(DemangleStyle::V0(d))
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1023:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (02a27c49f 2021-06-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc-demangle`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:601 ~ alloc[6322]::borrow::{impl#14}::add), const_param_did: None }) (end of phase Optimization) at bb1[6]:
bad index (isize != usize)
    |
443 |         self
    |         ^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:605 ~ alloc[6322]::borrow::{impl#15}::add), const_param_did: None }) (end of phase Optimization) at bb0[7]:
bad index (isize != usize)
    |
    |
454 |         self += rhs;
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:605 ~ alloc[6322]::borrow::{impl#15}::add), const_param_did: None }) (end of phase Optimization) at bb1[6]:
bad index (isize != usize)
    |
455 |         self
    |         ^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:814 ~ alloc[6322]::collections::binary_heap::{impl#26}::size_hint), const_param_did: None }) (end of phase Optimization) at bb1[13]:
bad index (isize != usize)
     |
     |
1342 |         (exact, Some(exact))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:853 ~ alloc[6322]::collections::binary_heap::{impl#35}::size_hint), const_param_did: None }) (end of phase Optimization) at bb1[13]:
bad index (isize != usize)
     |
     |
1444 |         (exact, Some(exact))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1396 ~ alloc[6322]::collections::btree::map::{impl#15}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[12]:
bad index (isize != usize)
     |
     |
1307 |         (self.length, Some(self.length))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1429 ~ alloc[6322]::collections::btree::map::{impl#21}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[12]:
bad index (isize != usize)
     |
     |
1376 |         (self.length, Some(self.length))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1483 ~ alloc[6322]::collections::btree::map::{impl#29}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[12]:
bad index (isize != usize)
     |
     |
1500 |         (self.length, Some(self.length))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1588 ~ alloc[6322]::collections::btree::map::{impl#46}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[10]:
bad index (isize != usize)
     |
     |
1718 |         (0, Some(*self.length))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1924 ~ alloc[6322]::collections::btree::navigate::{impl#8}::next_unchecked::{closure#0}), const_param_did: None }) (end of phase Optimization) at bb1[7]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:322:31
    |
322 |             let kv = unsafe { kv.ok().unwrap_unchecked() };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1926 ~ alloc[6322]::collections::btree::navigate::{impl#8}::next_back_unchecked::{closure#0}), const_param_did: None }) (end of phase Optimization) at bb1[7]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:335:31
    |
335 |             let kv = unsafe { kv.ok().unwrap_unchecked() };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1932 ~ alloc[6322]::collections::btree::navigate::{impl#9}::next_unchecked::{closure#0}), const_param_did: None }) (end of phase Optimization) at bb1[7]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:350:31
    |
350 |             let kv = unsafe { kv.ok().unwrap_unchecked() };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1934 ~ alloc[6322]::collections::btree::navigate::{impl#9}::next_back_unchecked::{closure#0}), const_param_did: None }) (end of phase Optimization) at bb1[7]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:365:31
    |
365 |             let kv = unsafe { kv.ok().unwrap_unchecked() };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1965 ~ alloc[6322]::collections::btree::navigate::{impl#12}::visit_nodes_in_order), const_param_did: None }) (end of phase Optimization) at bb2[15]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:460:17
    |
460 |                 visit(Position::Internal(internal));
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1965 ~ alloc[6322]::collections::btree::navigate::{impl#12}::visit_nodes_in_order), const_param_did: None }) (end of phase Optimization) at bb4[14]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:458:27
    |
458 |             Leaf(leaf) => visit(Position::Leaf(leaf)),
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1965 ~ alloc[6322]::collections::btree::navigate::{impl#12}::visit_nodes_in_order), const_param_did: None }) (end of phase Optimization) at bb11[15]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:475:29
    |
475 | ...                   visit(Position::Internal(internal));
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1965 ~ alloc[6322]::collections::btree::navigate::{impl#12}::visit_nodes_in_order), const_param_did: None }) (end of phase Optimization) at bb13[15]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:465:29
    |
465 | ...                   visit(Position::Leaf(leaf));
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1965 ~ alloc[6322]::collections::btree::navigate::{impl#12}::visit_nodes_in_order), const_param_did: None }) (end of phase Optimization) at bb18[15]:
bad index (isize != usize)
   --> library/alloc/src/collections/btree/navigate.rs:468:37
    |
468 | ...                   visit(Position::InternalKV(kv));
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2805 ~ alloc[6322]::collections::btree::set::{impl#6}::difference), const_param_did: None }) (end of phase Optimization) at bb6[7]:
bad index (isize != usize)
    |
    |
309 |                 return Difference { inner: DifferenceInner::Iterate(self.iter()) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2805 ~ alloc[6322]::collections::btree::set::{impl#6}::difference), const_param_did: None }) (end of phase Optimization) at bb12[7]:
bad index (isize != usize)
    |
    |
315 |                 return Difference { inner: DifferenceInner::Iterate(self.iter()) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2805 ~ alloc[6322]::collections::btree::set::{impl#6}::difference), const_param_did: None }) (end of phase Optimization) at bb35[3]:
bad index (isize != usize)
    |
317 | /         Difference {
317 | /         Difference {
318 | |             inner: match (self_min.cmp(other_max), self_max.cmp(other_min)) {
319 | |                 (Greater, _) | (_, Less) => DifferenceInner::Iterate(self.iter()),
320 | |                 (Equal, _) => {
337 | |             },
338 | |         }
    | |_________^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2809 ~ alloc[6322]::collections::btree::set::{impl#6}::intersection), const_param_did: None }) (end of phase Optimization) at bb3[6]:
bad index (isize != usize)
    |
    |
398 |                 return Intersection { inner: IntersectionInner::Answer(None) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2809 ~ alloc[6322]::collections::btree::set::{impl#6}::intersection), const_param_did: None }) (end of phase Optimization) at bb8[6]:
bad index (isize != usize)
    |
    |
404 |                 return Intersection { inner: IntersectionInner::Answer(None) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2809 ~ alloc[6322]::collections::btree::set::{impl#6}::intersection), const_param_did: None }) (end of phase Optimization) at bb32[3]:
bad index (isize != usize)
    |
406 | /         Intersection {
406 | /         Intersection {
407 | |             inner: match (self_min.cmp(other_max), self_max.cmp(other_min)) {
408 | |                 (Greater, _) | (_, Less) => IntersectionInner::Answer(None),
409 | |                 (Equal, _) => IntersectionInner::Answer(Some(self_min)),
418 | |             },
419 | |         }
    | |_________^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2981 ~ alloc[6322]::collections::btree::set::{impl#35}::clone), const_param_did: None }) (end of phase Optimization) at bb9[3]:
bad index (isize != usize)
     |
1393 | /         Difference {
1393 | /         Difference {
1394 | |             inner: match &self.inner {
1395 | |                 DifferenceInner::Stitch { self_iter, other_iter } => DifferenceInner::Stitch {
1396 | |                     self_iter: self_iter.clone(),
1403 | |             },
1404 | |         }
     | |_________^
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2988 ~ alloc[6322]::collections::btree::set::{impl#36}::size_hint), const_param_did: None }) (end of phase Optimization) at bb11[12]:
bad index (isize != usize)
     |
     |
1446 |         (self_len.saturating_sub(other_len), Some(self_len))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3000 ~ alloc[6322]::collections::btree::set::{impl#39}::size_hint), const_param_did: None }) (end of phase Optimization) at bb1[22]:
bad index (isize != usize)
     |
     |
1481 |         (0, Some(a_len + b_len))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3006 ~ alloc[6322]::collections::btree::set::{impl#41}::clone), const_param_did: None }) (end of phase Optimization) at bb8[3]:
bad index (isize != usize)
     |
1495 | /         Intersection {
1495 | /         Intersection {
1496 | |             inner: match &self.inner {
1497 | |                 IntersectionInner::Stitch { a, b } => {
1498 | |                     IntersectionInner::Stitch { a: a.clone(), b: b.clone() }
1504 | |             },
1505 | |         }
     | |_________^
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3012 ~ alloc[6322]::collections::btree::set::{impl#42}::size_hint), const_param_did: None }) (end of phase Optimization) at bb2[7]:
bad index (isize != usize)
     |
     |
1540 |             IntersectionInner::Answer(Some(_)) => (1, Some(1)),
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3012 ~ alloc[6322]::collections::btree::set::{impl#42}::size_hint), const_param_did: None }) (end of phase Optimization) at bb7[9]:
bad index (isize != usize)
     |
     |
1537 |             IntersectionInner::Stitch { a, b } => (0, Some(min(a.len(), b.len()))),
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3012 ~ alloc[6322]::collections::btree::set::{impl#42}::size_hint), const_param_did: None }) (end of phase Optimization) at bb9[8]:
bad index (isize != usize)
     |
     |
1538 |             IntersectionInner::Search { small_iter, .. } => (0, Some(small_iter.len())),
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3012 ~ alloc[6322]::collections::btree::set::{impl#42}::size_hint), const_param_did: None }) (end of phase Optimization) at bb10[7]:
bad index (isize != usize)
     |
     |
1539 |             IntersectionInner::Answer(None) => (0, Some(0)),
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3024 ~ alloc[6322]::collections::btree::set::{impl#45}::size_hint), const_param_did: None }) (end of phase Optimization) at bb2[18]:
bad index (isize != usize)
     |
     |
1570 |         (max(a_len, b_len), Some(a_len + b_len))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3194 ~ alloc[6322]::collections::linked_list::{impl#9}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[12]:
bad index (isize != usize)
     |
     |
1009 |         (self.len, Some(self.len))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3211 ~ alloc[6322]::collections::linked_list::{impl#13}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[12]:
bad index (isize != usize)
     |
     |
1063 |         (self.len, Some(self.len))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3295 ~ alloc[6322]::collections::linked_list::{impl#23}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[16]:
bad index (isize != usize)
     |
     |
1549 |         (0, Some(self.old_len - self.idx))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3321 ~ alloc[6322]::collections::linked_list::{impl#26}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[12]:
bad index (isize != usize)
     |
     |
1601 |         (self.list.len, Some(self.list.len))
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3610 ~ alloc[6322]::collections::vec_deque::iter_mut::{impl#3}::size_hint), const_param_did: None }) (end of phase Optimization) at bb2[15]:
bad index (isize != usize)
  --> library/alloc/src/collections/vec_deque/iter_mut.rs:61:9
   |
61 |         (len, Some(len))
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3661 ~ alloc[6322]::collections::vec_deque::into_iter::{impl#1}::size_hint), const_param_did: None }) (end of phase Optimization) at bb1[13]:
bad index (isize != usize)
  --> library/alloc/src/collections/vec_deque/into_iter.rs:37:9
   |
37 |         (len, Some(len))
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3723 ~ alloc[6322]::collections::vec_deque::iter::{impl#2}::size_hint), const_param_did: None }) (end of phase Optimization) at bb2[15]:
bad index (isize != usize)
  --> library/alloc/src/collections/vec_deque/iter.rs:53:9
   |
53 |         (len, Some(len))
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4613 ~ alloc[6322]::rc::{impl#42}::to_rc_slice), const_param_did: None }) (end of phase Optimization) at bb1[7]:
bad index (isize != usize)
     |
     |
1959 |         let (low, high) = self.size_hint();
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5272 ~ alloc[6322]::string::{impl#0}::from_utf16_lossy::{closure#0}), const_param_did: None }) (end of phase Optimization) at bb0[4]:
bad index (isize != usize)
    |
    |
650 |         decode_utf16(v.iter().cloned()).map(|r| r.unwrap_or(REPLACEMENT_CHARACTER)).collect()
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5323 ~ alloc[6322]::string::{impl#0}::replace_range), const_param_did: None }) (end of phase Optimization) at bb29[7]:
bad index (isize != usize)
     |
     |
1705 |         unsafe { self.as_mut_vec() }.splice((start, end), replace_with.bytes());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5323 ~ alloc[6322]::string::{impl#0}::replace_range), const_param_did: None }) (end of phase Optimization) at bb29[18]:
bad index (isize != usize)
     |
     |
1705 |         unsafe { self.as_mut_vec() }.splice((start, end), replace_with.bytes());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5323 ~ alloc[6322]::string::{impl#0}::replace_range), const_param_did: None }) (end of phase Optimization) at bb29[28]:
bad index (isize != usize)
     |
     |
1705 |         unsafe { self.as_mut_vec() }.splice((start, end), replace_with.bytes());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5323 ~ alloc[6322]::string::{impl#0}::replace_range), const_param_did: None }) (end of phase Optimization) at bb29[38]:
bad index (isize != usize)
     |
     |
1705 |         unsafe { self.as_mut_vec() }.splice((start, end), replace_with.bytes());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5356 ~ alloc[6322]::string::{impl#10}::from_iter), const_param_did: None }) (end of phase Optimization) at bb3[4]:
bad index (isize != usize)
     |
     |
1900 |             Some(cow) => {
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5356 ~ alloc[6322]::string::{impl#10}::from_iter), const_param_did: None }) (end of phase Optimization) at bb3[16]:
bad index (isize != usize)
     |
     |
1901 |                 let mut buf = cow.into_owned();
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:121:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5953 ~ alloc[6322]::sync::{impl#55}::to_arc_slice), const_param_did: None }) (end of phase Optimization) at bb1[7]:
