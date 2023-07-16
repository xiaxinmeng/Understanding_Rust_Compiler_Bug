plain
   Compiling gimli v0.23.0
   Compiling hashbrown v0.9.0
   Compiling miniz_oxide v0.4.0
   Compiling object v0.22.0
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:29 ~ alloc[be1d]::alloc::{impl#0}::grow_impl), const_param_did: None }) (end of phase Optimization) at bb40[3]:
bad arg (*const u8 != *mut u8)
    |
    |
213 |                 ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), old_size);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:36 ~ alloc[be1d]::alloc::{impl#1}::shrink), const_param_did: None }) (end of phase Optimization) at bb36[3]:
bad arg (*const u8 != *mut u8)
    |
    |
301 |                 ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), new_size);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:235 ~ alloc[be1d]::boxed::{impl#22}::from), const_param_did: None }) (end of phase Optimization) at bb4[3]:
bad arg (*const T != *mut T)
     |
     |
1236 |             ptr::copy_nonoverlapping(slice.as_ptr(), buf.ptr(), len);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:607 ~ alloc[be1d]::collections::binary_heap::{impl#10}::move_to), const_param_did: None }) (end of phase Optimization) at bb8[6]:
bad arg (*const T != *mut T)
     |
     |
1125 |             ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:610 ~ alloc[be1d]::collections::binary_heap::{impl#11}::drop), const_param_did: None }) (end of phase Optimization) at bb2[3]:
bad arg (*const T != *mut T)
     |
     |
1137 |             ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2256 ~ alloc[be1d]::collections::btree::node::move_to_slice), const_param_did: None }) (end of phase Optimization) at bb7[1]:
bad arg (*const core::mem::MaybeUninit<T> != *mut core::mem::MaybeUninit<T>)
    --> library/alloc/src/collections/btree/node.rs:1714:9
     |
1714 |         ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3283 ~ alloc[be1d]::collections::vec_deque::{impl#3}::copy_nonoverlapping), const_param_did: None }) (end of phase Optimization) at bb22[4]:
bad arg (*const T != *mut T)
    |
    |
265 |             ptr::copy_nonoverlapping(self.ptr().add(src), self.ptr().add(dst), len);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3342 ~ alloc[be1d]::collections::vec_deque::{impl#4}::split_off), const_param_did: None }) (end of phase Optimization) at bb12[3]:
bad arg (*const T != *mut T)
     |
     |
1985 |                 ptr::copy_nonoverlapping(first_half.as_ptr().add(at), other.ptr(), amount_in_first);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3342 ~ alloc[be1d]::collections::vec_deque::{impl#4}::split_off), const_param_did: None }) (end of phase Optimization) at bb15[4]:
bad arg (*const T != *mut T)
     |
1988 | /                 ptr::copy_nonoverlapping(
1988 | /                 ptr::copy_nonoverlapping(
1989 | |                     second_half.as_ptr(),
1990 | |                     other.ptr().add(amount_in_first),
1991 | |                     second_len,
1992 | |                 );
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3342 ~ alloc[be1d]::collections::vec_deque::{impl#4}::split_off), const_param_did: None }) (end of phase Optimization) at bb18[3]:
bad arg (*const T != *mut T)
     |
1998 | /                 ptr::copy_nonoverlapping(
1998 | /                 ptr::copy_nonoverlapping(
1999 | |                     second_half.as_ptr().add(offset),
2000 | |                     other.ptr(),
2001 | |                     amount_in_second,
2002 | |                 );
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3349 ~ alloc[be1d]::collections::vec_deque::{impl#4}::make_contiguous), const_param_did: None }) (end of phase Optimization) at bb13[8]:
bad arg (*const T != *mut T)
     |
     |
2217 |                 ptr::copy_nonoverlapping(buf.add(self.tail), buf, tail_len);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3349 ~ alloc[be1d]::collections::vec_deque::{impl#4}::make_contiguous), const_param_did: None }) (end of phase Optimization) at bb19[4]:
bad arg (*const T != *mut T)
     |
     |
2239 |                 ptr::copy_nonoverlapping(buf, buf.add(self.head + tail_len), self.head);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3716 ~ alloc[be1d]::rc::{impl#12}::from_box), const_param_did: None }) (end of phase Optimization) at bb4[20]:
bad arg (*const u8 != *mut u8)
     |
1201 | /             ptr::copy_nonoverlapping(
1201 | /             ptr::copy_nonoverlapping(
1202 | |                 bptr as *const T as *const u8,
1203 | |                 &mut (*ptr).value as *mut _ as *mut u8,
1205 | |             );
     | |_____________^
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3722 ~ alloc[be1d]::rc::{impl#13}::copy_from_slice), const_param_did: None }) (end of phase Optimization) at bb4[1]:
bad arg (*const T != *mut T)
     |
     |
1233 |             ptr::copy_nonoverlapping(v.as_ptr(), &mut (*ptr).value as *mut [T] as *mut T, v.len());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4026 ~ alloc[be1d]::slice::{impl#0}::repeat), const_param_did: None }) (end of phase Optimization) at bb15[1]:
bad arg (*const T != *mut T)
    |
547 | /                     ptr::copy_nonoverlapping(
548 | |                         buf.as_ptr(),
548 | |                         buf.as_ptr(),
549 | |                         (buf.as_mut_ptr() as *mut T).add(buf.len()),
550 | |                         buf.len(),
551 | |                     );
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4026 ~ alloc[be1d]::slice::{impl#0}::repeat), const_param_did: None }) (end of phase Optimization) at bb24[4]:
bad arg (*const T != *mut T)
    |
568 | /                 ptr::copy_nonoverlapping(
569 | |                     buf.as_ptr(),
569 | |                     buf.as_ptr(),
570 | |                     (buf.as_mut_ptr() as *mut T).add(buf.len()),
571 | |                     rem_len,
572 | |                 );
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4073 ~ alloc[be1d]::slice::insert_head), const_param_did: None }) (end of phase Optimization) at bb17[2]:
bad arg (*const T != *mut T)
    |
    |
875 |             ptr::copy_nonoverlapping(&v[1], &mut v[0], 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4073 ~ alloc[be1d]::slice::insert_head), const_param_did: None }) (end of phase Optimization) at bb31[2]:
bad arg (*const T != *mut T)
    |
    |
881 |                 ptr::copy_nonoverlapping(&v[i], &mut v[i - 1], 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4082 ~ alloc[be1d]::slice::insert_head::{impl#0}::drop), const_param_did: None }) (end of phase Optimization) at bb0[7]:
bad arg (*const T != *mut T)
    |
    |
897 |                 ptr::copy_nonoverlapping(self.src, self.dest, 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4083 ~ alloc[be1d]::slice::merge), const_param_did: None }) (end of phase Optimization) at bb5[9]:
bad arg (*const T != *mut T)
    |
    |
940 |             ptr::copy_nonoverlapping(v, buf, mid);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4083 ~ alloc[be1d]::slice::merge), const_param_did: None }) (end of phase Optimization) at bb6[15]:
bad arg (*const T != *mut T)
    |
    |
964 |             ptr::copy_nonoverlapping(v_mid, buf, len - mid);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4083 ~ alloc[be1d]::slice::merge), const_param_did: None }) (end of phase Optimization) at bb21[1]:
bad arg (*const T != *mut T)
    |
    |
958 |                 ptr::copy_nonoverlapping(to_copy, get_and_increment(out), 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4083 ~ alloc[be1d]::slice::merge), const_param_did: None }) (end of phase Optimization) at bb38[1]:
bad arg (*const T != *mut T)
    |
    |
982 |                 ptr::copy_nonoverlapping(to_copy, decrement_and_get(&mut out), 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4097 ~ alloc[be1d]::slice::merge::{impl#0}::drop), const_param_did: None }) (end of phase Optimization) at bb2[12]:
bad arg (*const T != *mut T)
     |
     |
1012 |                 ptr::copy_nonoverlapping(self.start, self.dest, len);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4666 ~ alloc[be1d]::sync::{impl#17}::from_box), const_param_did: None }) (end of phase Optimization) at bb4[20]:
bad arg (*const u8 != *mut u8)
     |
1143 | /             ptr::copy_nonoverlapping(
1143 | /             ptr::copy_nonoverlapping(
1144 | |                 bptr as *const T as *const u8,
1145 | |                 &mut (*ptr).data as *mut _ as *mut u8,
1147 | |             );
     | |_____________^
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4672 ~ alloc[be1d]::sync::{impl#18}::copy_from_slice), const_param_did: None }) (end of phase Optimization) at bb4[1]:
bad arg (*const T != *mut T)
     |
     |
1176 |             ptr::copy_nonoverlapping(v.as_ptr(), &mut (*ptr).data as *mut [T] as *mut T, v.len());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4927 ~ alloc[be1d]::vec::drain_filter::{impl#1}::next), const_param_did: None }) (end of phase Optimization) at bb15[7]:
bad arg (*const T != *mut T)
   |
   |
85 |                     ptr::copy_nonoverlapping(src, dst, 1);
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5440 ~ alloc[be1d]::vec::{impl#1}::retain), const_param_did: None }) (end of phase Optimization) at bb15[6]:
bad arg (*const T != *mut T)
     |
     |
1464 |                     ptr::copy_nonoverlapping(cur, hole_slot, 1);
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5463 ~ alloc[be1d]::vec::{impl#1}::append_elements), const_param_did: None }) (end of phase Optimization) at bb5[4]:
bad arg (*const T != *mut T)
     |
     |
1609 |         unsafe { ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(len), count) };
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5469 ~ alloc[be1d]::vec::{impl#1}::split_off), const_param_did: None }) (end of phase Optimization) at bb20[1]:
bad arg (*const T != *mut T)
     |
     |
1774 |             ptr::copy_nonoverlapping(self.as_ptr().add(at), other.as_mut_ptr(), other.len());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:5536 ~ alloc[be1d]::vec::{impl#9}::spec_extend_from_within), const_param_did: None }) (end of phase Optimization) at bb5[6]:
bad arg (*const T != *mut T)
     |
     |
2204 |             unsafe { ptr::copy_nonoverlapping(source.as_ptr(), spare.as_mut_ptr() as _, count) };
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1012:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (a28aff00f 2021-02-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1644 ~ core[8a3d]::ptr::const_ptr::{impl#0}::copy_to_nonoverlapping), const_param_did: None }) (end of phase Optimization) at bb0[6]:
bad arg (*const T != *mut T)
    |
    |
841 |         unsafe { copy_nonoverlapping(self, dest, count) }
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1708 ~ core[8a3d]::ptr::mut_ptr::{impl#0}::copy_to_nonoverlapping), const_param_did: None }) (end of phase Optimization) at bb0[9]:
bad arg (*const T != *mut T)
    |
    |
948 |         unsafe { copy_nonoverlapping(self, dest, count) }
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1710 ~ core[8a3d]::ptr::mut_ptr::{impl#0}::copy_from_nonoverlapping), const_param_did: None }) (end of phase Optimization) at bb0[6]:
bad arg (*const T != *mut T)
    |
    |
984 |         unsafe { copy_nonoverlapping(src, self, count) }
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1754 ~ core[8a3d]::ptr::swap), const_param_did: None }) (end of phase Optimization) at bb2[1]:
bad arg (*const T != *mut T)
    |
    |
408 |         copy_nonoverlapping(x, tmp.as_mut_ptr(), 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1754 ~ core[8a3d]::ptr::swap), const_param_did: None }) (end of phase Optimization) at bb4[3]:
bad arg (*const T != *mut T)
    |
    |
410 |         copy_nonoverlapping(tmp.as_ptr(), y, 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1758 ~ core[8a3d]::ptr::swap_nonoverlapping_one), const_param_did: None }) (end of phase Optimization) at bb4[8]:
bad arg (*const T != *mut T)
    |
    |
481 |             copy_nonoverlapping(y, x, 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1760 ~ core[8a3d]::ptr::swap_nonoverlapping_bytes), const_param_did: None }) (end of phase Optimization) at bb8[11]:
bad arg (*const u8 != *mut u8)
    |
    |
526 |             copy_nonoverlapping(x, t, block_size);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1760 ~ core[8a3d]::ptr::swap_nonoverlapping_bytes), const_param_did: None }) (end of phase Optimization) at bb8[24]:
bad arg (*const u8 != *mut u8)
    |
    |
527 |             copy_nonoverlapping(y, x, block_size);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1760 ~ core[8a3d]::ptr::swap_nonoverlapping_bytes), const_param_did: None }) (end of phase Optimization) at bb8[37]:
bad arg (*const u8 != *mut u8)
    |
    |
528 |             copy_nonoverlapping(t, y, block_size);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1760 ~ core[8a3d]::ptr::swap_nonoverlapping_bytes), const_param_did: None }) (end of phase Optimization) at bb14[11]:
bad arg (*const u8 != *mut u8)
    |
    |
545 |             copy_nonoverlapping(x, t, rem);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1760 ~ core[8a3d]::ptr::swap_nonoverlapping_bytes), const_param_did: None }) (end of phase Optimization) at bb14[24]:
bad arg (*const u8 != *mut u8)
    |
    |
546 |             copy_nonoverlapping(y, x, rem);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1760 ~ core[8a3d]::ptr::swap_nonoverlapping_bytes), const_param_did: None }) (end of phase Optimization) at bb14[37]:
bad arg (*const u8 != *mut u8)
    |
    |
547 |             copy_nonoverlapping(t, y, rem);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1775 ~ core[8a3d]::ptr::read), const_param_did: None }) (end of phase Optimization) at bb2[1]:
bad arg (*const T != *mut T)
    |
    |
720 |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1777 ~ core[8a3d]::ptr::read_unaligned), const_param_did: None }) (end of phase Optimization) at bb3[0]:
bad arg (*const u8 != *mut u8)
    |
    |
819 |         copy_nonoverlapping(src as *const u8, tmp.as_mut_ptr() as *mut u8, mem::size_of::<T>());
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1779 ~ core[8a3d]::ptr::write), const_param_did: None }) (end of phase Optimization) at bb0[6]:
bad arg (*const T != *mut T)
    |
    |
911 |         copy_nonoverlapping(&src as *const T, dst, 1);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1781 ~ core[8a3d]::ptr::write_unaligned), const_param_did: None }) (end of phase Optimization) at bb1[0]:
bad arg (*const u8 != *mut u8)
     |
     |
1008 |         copy_nonoverlapping(&src as *const T as *const u8, dst as *mut u8, mem::size_of::<T>());
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:6956 ~ core[8a3d]::fmt::num::parse_u64_into), const_param_did: None }) (end of phase Optimization) at bb8[2]:
bad arg (*const u8 != *mut u8)
   --> library/core/src/fmt/num.rs:509:13
    |
509 |             ptr::copy_nonoverlapping(lut_ptr.offset(d1 as isize), buf_ptr.offset(*curr + 0), 2);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:6956 ~ core[8a3d]::fmt::num::parse_u64_into), const_param_did: None }) (end of phase Optimization) at bb10[2]:
bad arg (*const u8 != *mut u8)
   --> library/core/src/fmt/num.rs:510:13
    |
510 |             ptr::copy_nonoverlapping(lut_ptr.offset(d2 as isize), buf_ptr.offset(*curr + 2), 2);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:6956 ~ core[8a3d]::fmt::num::parse_u64_into), const_param_did: None }) (end of phase Optimization) at bb12[2]:
bad arg (*const u8 != *mut u8)
   --> library/core/src/fmt/num.rs:511:13
    |
511 |             ptr::copy_nonoverlapping(lut_ptr.offset(d3 as isize), buf_ptr.offset(*curr + 4), 2);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:6956 ~ core[8a3d]::fmt::num::parse_u64_into), const_param_did: None }) (end of phase Optimization) at bb14[2]:
