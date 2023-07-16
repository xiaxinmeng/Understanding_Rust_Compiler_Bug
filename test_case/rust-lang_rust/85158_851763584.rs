plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.11.0
   Compiling object v0.22.0
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2805 ~ alloc[6322]::collections::btree::set::{impl#6}::difference), const_param_did: None }) (end of phase Optimization) at bb6[12]:
bad arg (u8 != collections::btree::set::DifferenceInner<T>)
    |
    |
309 |                 return Difference { inner: DifferenceInner::Iterate(self.iter()) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2805 ~ alloc[6322]::collections::btree::set::{impl#6}::difference), const_param_did: None }) (end of phase Optimization) at bb12[12]:
bad arg (u8 != collections::btree::set::DifferenceInner<T>)
    |
    |
315 |                 return Difference { inner: DifferenceInner::Iterate(self.iter()) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2805 ~ alloc[6322]::collections::btree::set::{impl#6}::difference), const_param_did: None }) (end of phase Optimization) at bb35[8]:
bad arg (u8 != collections::btree::set::DifferenceInner<T>)
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
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2809 ~ alloc[6322]::collections::btree::set::{impl#6}::intersection), const_param_did: None }) (end of phase Optimization) at bb3[11]:
bad arg (u8 != collections::btree::set::IntersectionInner<T>)
    |
    |
398 |                 return Intersection { inner: IntersectionInner::Answer(None) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2809 ~ alloc[6322]::collections::btree::set::{impl#6}::intersection), const_param_did: None }) (end of phase Optimization) at bb8[11]:
bad arg (u8 != collections::btree::set::IntersectionInner<T>)
    |
    |
404 |                 return Intersection { inner: IntersectionInner::Answer(None) };
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2809 ~ alloc[6322]::collections::btree::set::{impl#6}::intersection), const_param_did: None }) (end of phase Optimization) at bb32[8]:
bad arg (u8 != collections::btree::set::IntersectionInner<T>)
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
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2981 ~ alloc[6322]::collections::btree::set::{impl#35}::clone), const_param_did: None }) (end of phase Optimization) at bb9[8]:
bad arg (u8 != collections::btree::set::DifferenceInner<T>)
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
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3006 ~ alloc[6322]::collections::btree::set::{impl#41}::clone), const_param_did: None }) (end of phase Optimization) at bb8[8]:
bad arg (u8 != collections::btree::set::IntersectionInner<T>)
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
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1021:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (391e44947 2021-06-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1184 ~ gimli[3066]::read::abbrev::{impl#6}::parse_attributes), const_param_did: None }) (end of phase Optimization) at bb9[11]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:267:12
267 |         Ok(attrs)
    |            ^^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1184 ~ gimli[3066]::read::abbrev::{impl#6}::parse_attributes), const_param_did: None }) (end of phase Optimization) at bb9[21]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:267:9
267 |         Ok(attrs)
    |         ^^^^^^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1186 ~ gimli[3066]::read::abbrev::{impl#6}::parse), const_param_did: None }) (end of phase Optimization) at bb23[9]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:280:26
280 |         let attributes = Self::parse_attributes(input)?;
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1186 ~ gimli[3066]::read::abbrev::{impl#6}::parse), const_param_did: None }) (end of phase Optimization) at bb23[20]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:280:26
280 |         let attributes = Self::parse_attributes(input)?;
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1186 ~ gimli[3066]::read::abbrev::{impl#6}::parse), const_param_did: None }) (end of phase Optimization) at bb23[41]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:281:65
    |
281 |         let abbrev = Abbreviation::new(code, tag, has_children, attributes);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:6008 ~ gimli[3066]::read::abbrev::{impl#23}::clone), const_param_did: None }) (end of phase Optimization) at bb4[12]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:184:17
    |
184 |   #[derive(Debug, Clone, PartialEq, Eq)]
    |                   ^^^^^ in this macro invocation
   ::: /checkout/library/core/src/clone.rs:139:1
    |
    |
139 | / pub macro Clone($item:item) {
141 | | }
141 | | }
    | |_- in this expansion of `#[derive(Clone)]`
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1175 ~ gimli[3066]::read::abbrev::{impl#6}::new), const_param_did: None }) (end of phase Optimization) at bb2[19]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:209:13
209 |             attributes,
    |             ^^^^^^^^^^
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1175 ~ gimli[3066]::read::abbrev::{impl#6}::new), const_param_did: None }) (end of phase Optimization) at bb2[32]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:205:9
205 | /         Abbreviation {
206 | |             code,
207 | |             tag,
208 | |             has_children,
208 | |             has_children,
209 | |             attributes,
210 | |         }
    | |_________^
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1192 ~ gimli[3066]::read::abbrev::{impl#7}::push), const_param_did: None }) (end of phase Optimization) at bb12[8]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:320:17
    |
320 |                 *self = Attributes::Heap(list);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1192 ~ gimli[3066]::read::abbrev::{impl#7}::push), const_param_did: None }) (end of phase Optimization) at bb13[8]:
bad arg (u8 != read::abbrev::Attributes)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:320:17
    |
320 |                 *self = Attributes::Heap(list);
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1021:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (391e44947 2021-06-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:360 ~ object[5791]::read::any::{impl#0}::parse), const_param_did: None }) (end of phase Optimization) at bb80[16]:
bad arg (u8 != read::any::FileInternal)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.22.0/src/read/any.rs:217:19
    |
217 |         Ok(File { inner })
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:360 ~ object[5791]::read::any::{impl#0}::parse), const_param_did: None }) (end of phase Optimization) at bb80[26]:
bad arg (u8 != read::any::FileInternal)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.22.0/src/read/any.rs:217:12
    |
217 |         Ok(File { inner })
    |
    |
    = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1021:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (391e44947 2021-06-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
