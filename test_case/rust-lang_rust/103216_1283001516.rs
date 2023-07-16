plain
   |
57 |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
   |
   = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
help: indicate the anonymous lifetime
   |
57 |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+)<'_>)

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panic.rs:57:38
   |
   |
57 |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
57 |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+)<'_>)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/macros/mod.rs:520:24
    |
    |
520 |         $dst.write_fmt($crate::format_args!($($arg)*))
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
520 |         $dst.write_fmt($crate::format_args!($($arg)*)<'_>)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/bignum.rs:415:36
    |
    |
415 |                 write!(f, "{:#x}", self.base[sz - 1])?;
    |                                    ^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
415 |                 write!(f, "{:#x}", self.base[sz - 1]<'_>)?;

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/bignum.rs:417:43
    |
    |
417 |                     write!(f, "_{:01$x}", v, digitlen)?;
    |                                           ^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
417 |                     write!(f, "_{:01$x}", v<'_>, digitlen)?;

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/bignum.rs:417:46
    |
    |
417 |                     write!(f, "_{:01$x}", v, digitlen)?;
    |                                              ^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
417 |                     write!(f, "_{:01$x}", v, digitlen<'_>)?;

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/dec2flt/common.rs:114:10
    |
    |
114 | #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
114 | #[derive(Debug<'_>, Clone, Copy, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/dec2flt/common.rs:186:10
    |
    |
186 | #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
186 | #[derive(Debug<'_>, Copy, Clone, PartialEq, Eq, Default)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/dec2flt/number.rs:26:23
   |
   |
26 | #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
26 | #[derive(Clone, Copy, Debug<'_>, Default, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/dec2flt/parse.rs:110:17
    |
    |
110 |     let mut s = AsciiStr::new(s);
    |                 |
    |                 expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
110 |     let mut s = AsciiStr<'_>::new(s);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/dec2flt/mod.rs:173:10
    |
    |
173 | #[derive(Debug, Clone, PartialEq, Eq)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
173 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/dec2flt/mod.rs:179:10
    |
    |
179 | #[derive(Debug, Clone, PartialEq, Eq)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
179 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/diy_float.rs:13:23
   |
   |
13 | #[derive(Copy, Clone, Debug)]
   |                       ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
13 | #[derive(Copy, Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/flt2dec/decoder.rs:13:23
   |
   |
13 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
13 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/flt2dec/decoder.rs:30:23
   |
   |
30 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
30 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:300:38
    |
    |
300 | #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
300 | #[derive(Copy, Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:370:13
    |
    |
370 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
370 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:375:13
    |
    |
375 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
375 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:382:17
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:398:13
    |
398 |             Formatted { sign, parts: digits_to_dec_str(buf, exp, frac_digits, parts) }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
398 |             Formatted<'_> { sign, parts: digits_to_dec_str(buf, exp, frac_digits, parts) }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:446:13
    |
    |
446 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
446 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:451:13
    |
    |
451 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
451 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:460:13
    |
    |
460 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
460 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:470:13
    |
    |
470 |             Formatted { sign, parts }
    |             ^^^^^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
470 |             Formatted<'_> { sign, parts }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:537:13
    |
    |
537 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
537 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:542:13
    |
    |
542 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
542 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:550:17
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:570:13
    |
570 |             Formatted { sign, parts: digits_to_exp_str(buf, exp, ndigits, upper, parts) }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
570 |             Formatted<'_> { sign, parts: digits_to_exp_str(buf, exp, ndigits, upper, parts) }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:611:13
    |
    |
611 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
611 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:616:13
    |
    |
616 |             Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
    |             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
616 |             Formatted<'_> { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:623:17
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/num/flt2dec/mod.rs:669:17
    |
669 |                 Formatted { sign, parts: digits_to_dec_str(buf, exp, frac_digits, parts) }
    |                 ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
669 |                 Formatted<'_> { sign, parts: digits_to_dec_str(buf, exp, frac_digits, parts) }

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/fmt.rs:10:38
   |
   |
10 | #[derive(Copy, Clone, PartialEq, Eq, Debug)]
   |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
10 | #[derive(Copy, Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/fmt.rs:74:10
   |
---

error: hidden lifetime parameters in types are deprecated
 --> library/core/src/num/error.rs:9:10
  |
9 | #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
  |
  |
9 | #[derive(Debug<'_>, Copy, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/error.rs:68:10
   |
   |
68 | #[derive(Debug, Clone, PartialEq, Eq)]
   |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
68 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/num/error.rs:86:10
   |
   |
86 | #[derive(Debug, Clone, PartialEq, Eq)]
   |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
86 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/num/mod.rs:1014:38
     |
     |
1014 | #[derive(Copy, Clone, PartialEq, Eq, Debug)]
     |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1014 | #[derive(Copy, Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/num/mod.rs:1119:9
     |
     |
1119 |         radix
     |         ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1119 |         radix<'_>

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/mem/manually_drop.rs:48:23
   |
   |
48 | #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
48 | #[derive(Copy, Clone, Debug<'_>, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/macros/mod.rs:58:114
   |
   |
58 | ...ght_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
58 |                     $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)<'_>));

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/mem/maybe_uninit.rs:1152:25
     |
     |
1152 |         let mut guard = Guard { slice: this, initialized: 0 };
     |                         ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1152 |         let mut guard = Guard<'_> { slice: this, initialized: 0 };

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/mem/transmutability.rs:21:38
   |
   |
21 | #[derive(PartialEq, Eq, Clone, Copy, Debug)]
   |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
21 | #[derive(PartialEq, Eq, Clone, Copy, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ptr/alignment.rs:125:39
    |
    |
125 |         write!(f, "{:?} (1 << {:?})", self.as_nonzero(), self.log2())
    |                                       ^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
125 |         write!(f, "{:?} (1 << {:?})", self.as_nonzero()<'_>, self.log2())

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ptr/alignment.rs:125:58
    |
    |
125 |         write!(f, "{:?} (1 << {:?})", self.as_nonzero(), self.log2())
    |                                                          ^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
125 |         write!(f, "{:?} (1 << {:?})", self.as_nonzero(), self.log2()<'_>)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/cmp.rs:333:27
    |
    |
333 | #[derive(Clone, Copy, Eq, Debug, Hash)]
    |                           ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
333 | #[derive(Clone, Copy, Eq, Debug<'_>, Hash)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/cmp.rs:597:25
    |
    |
597 | #[derive(PartialEq, Eq, Debug, Copy, Default, Hash)]
    |                         ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
597 | #[derive(PartialEq, Eq, Debug<'_>, Copy, Default, Hash)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/error.rs:210:14
    |
    |
210 |     #[derive(Debug)]
    |              ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
210 |     #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/error.rs:410:9
    |
    |
410 |         Source { current: Some(self) }
    |         ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
410 |         Source<'_> { current: Some(self) }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/error.rs:419:10
    |
---
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
419 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/marker.rs:771:10
    |
    |
771 | #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
771 | #[derive(Debug<'_>, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/ops/control_flow.rs:82:10
   |
   |
82 | #[derive(Debug, Clone, Copy, PartialEq)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
82 | #[derive(Debug<'_>, Clone, Copy, PartialEq)]

error: hidden lifetime parameters in types are deprecated
 --> library/core/src/ops/generator.rs:9:55
  |
  |
9 | #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
  |                                                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
  |
  |
9 | #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug<'_>, Hash)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/ops/index_range.rs:10:17
   |
   |
10 | #[derive(Clone, Debug, PartialEq, Eq)]
   |                 ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
10 | #[derive(Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ops/range.rs:664:23
    |
    |
664 | #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
664 | #[derive(Clone, Copy, Debug<'_>, Hash, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ops/try_trait.rs:423:10
    |
    |
423 | #[derive(Debug)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
423 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/any.rs:665:55
    |
    |
665 | #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    |                                                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
665 | #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug<'_>, Hash)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/any.rs:1207:14
     |
     |
1207 |     #[derive(Debug)]
     |              ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1207 |     #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/any.rs:1215:14
     |
     |
1215 |     #[derive(Debug)]
     |              ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1215 |     #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/any.rs:1224:14
     |
     |
1224 |     #[derive(Debug)]
     |              ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1224 |     #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/any.rs:1242:9
     |
     |
1242 |         Demand::new(self as &mut (dyn Erased<'a> + 'a))
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1242 |         Demand<'_>::new(self as &mut (dyn Erased<'a> + 'a))

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/array/mod.rs:112:10
    |
    |
112 | #[derive(Debug, Copy, Clone)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
112 | #[derive(Debug<'_>, Copy, Clone)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/array/mod.rs:883:21
    |
    |
883 |     let mut guard = Guard { array_mut: &mut array, initialized: 0 };
    |                     ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
883 |     let mut guard = Guard<'_> { array_mut: &mut array, initialized: 0 };

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/async_iter/from_iter.rs:13:17
   |
   |
13 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
13 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/cell.rs:897:15
    |
    |
897 |         match BorrowRef::new(&self.borrow) {
    |               |
    |               expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
897 |         match BorrowRef<'_>::new(&self.borrow) {

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/cell.rs:910:20
    |
    |
910 |                 Ok(Ref { value, borrow: b })
    |                    ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
910 |                 Ok(Ref<'_> { value, borrow: b })

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/cell.rs:987:15
    |
    |
987 |         match BorrowRefMut::new(&self.borrow) {
    |               |
    |               expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
987 |         match BorrowRefMut<'_>::new(&self.borrow) {

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/cell.rs:996:20
    |
    |
996 |                 Ok(RefMut { value, borrow: b, marker: PhantomData })
    |                    ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
996 |                 Ok(RefMut<'_> { value, borrow: b, marker: PhantomData })

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1296:18
     |
     |
1296 |             Some(BorrowRef { borrow })
     |                  ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1296 |             Some(BorrowRef<'_> { borrow })

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1321:9
     |
     |
1321 |         BorrowRef { borrow: self.borrow }
     |         ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1321 |         BorrowRef<'_> { borrow: self.borrow }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1363:9
     |
     |
1363 |         Ref { value: orig.value, borrow: orig.borrow.clone() }
     |         ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1363 |         Ref<'_> { value: orig.value, borrow: orig.borrow.clone() }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1390:9
     |
     |
1390 |         Ref { value: NonNull::from(f(&*orig)), borrow: orig.borrow }
     |         ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1390 |         Ref<'_> { value: NonNull::from(f(&*orig)), borrow: orig.borrow }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1420:31
     |
     |
1420 |             Some(value) => Ok(Ref { value: NonNull::from(value), borrow: orig.borrow }),
     |                               ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1420 |             Some(value) => Ok(Ref<'_> { value: NonNull::from(value), borrow: orig.borrow }),

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1454:13
     |
     |
1454 |             Ref { value: NonNull::from(a), borrow },
     |             ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1454 |             Ref<'_> { value: NonNull::from(a), borrow },

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1455:13
     |
     |
1455 |             Ref { value: NonNull::from(b), borrow: orig.borrow },
     |             ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1455 |             Ref<'_> { value: NonNull::from(b), borrow: orig.borrow },

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1536:9
     |
     |
1536 |         RefMut { value, borrow: orig.borrow, marker: PhantomData }
     |         ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1536 |         RefMut<'_> { value, borrow: orig.borrow, marker: PhantomData }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1579:20
     |
     |
1579 |                 Ok(RefMut { value: NonNull::from(value), borrow: orig.borrow, marker: PhantomData })
     |                    ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1579 |                 Ok(RefMut<'_> { value: NonNull::from(value), borrow: orig.borrow, marker: PhantomData })

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1622:13
     |
     |
1622 |             RefMut { value: NonNull::from(a), borrow, marker: PhantomData },
     |             ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1622 |             RefMut<'_> { value: NonNull::from(a), borrow, marker: PhantomData },

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1623:13
     |
     |
1623 |             RefMut { value: NonNull::from(b), borrow: orig.borrow, marker: PhantomData },
     |             ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1623 |             RefMut<'_> { value: NonNull::from(b), borrow: orig.borrow, marker: PhantomData },

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1685:22
     |
     |
1685 |                 Some(BorrowRefMut { borrow })
     |                      ^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1685 |                 Some(BorrowRefMut<'_> { borrow })

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/cell.rs:1703:9
     |
     |
1703 |         BorrowRefMut { borrow: self.borrow }
     |         ^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1703 |         BorrowRefMut<'_> { borrow: self.borrow }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/convert.rs:149:17
    |
    |
149 | #[derive(Clone, Debug, PartialEq, Eq)]
    |                 ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
149 | #[derive(Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/convert.rs:169:23
    |
    |
169 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
169 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/convert.rs:235:23
    |
    |
235 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
235 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/char/decode.rs:15:17
   |
   |
15 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
15 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/char/decode.rs:28:10
   |
   |
28 | #[derive(Debug, Clone, Eq, PartialEq)]
   |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
28 | #[derive(Debug<'_>, Clone, Eq, PartialEq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/decode.rs:122:53
    |
    |
122 |         write!(f, "unpaired surrogate found: {:x}", self.code)
    |                                                     ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
122 |         write!(f, "unpaired surrogate found: {:x}", self.code<'_>)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/char/methods.rs:1752:13
     |
---
     |             ^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1754 |             dst.len()<'_>,

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/char/methods.rs:1789:17
     |
     |
1789 |                 from_u32_unchecked(code).len_utf16(),
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1789 |                 from_u32_unchecked(code).len_utf16()<'_>,

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/char/methods.rs:1790:17
     |
---
     |                 ^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1791 |                 dst.len()<'_>,

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:147:17
    |
    |
147 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
147 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:162:17
    |
    |
162 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
162 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:268:17
    |
    |
268 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
268 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:274:17
    |
    |
274 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
274 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:377:17
    |
    |
377 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
377 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:411:10
    |
    |
411 | #[derive(Debug, Clone)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
411 | #[derive(Debug<'_>, Clone)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:445:10
    |
    |
445 | #[derive(Debug, Clone)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
445 | #[derive(Debug<'_>, Clone)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:472:10
    |
    |
472 | #[derive(Debug, Clone)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
472 | #[derive(Debug<'_>, Clone)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/char/mod.rs:579:10
    |
    |
579 | #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
579 | #[derive(Debug<'_>, Copy, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:110:32
    |
    |
110 | #[derive(Clone, PartialEq, Eq, Debug)]
    |                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
110 | #[derive(Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:116:32
    |
    |
116 | #[derive(Clone, PartialEq, Eq, Debug)]
    |                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
116 | #[derive(Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:149:32
    |
    |
149 | #[derive(Clone, PartialEq, Eq, Debug)]
    |                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
149 | #[derive(Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:163:29
    |
    |
163 |         write!(f, "\"{}\"", self.to_bytes().escape_ascii())
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
163 |         write!(f, "\"{}\"", self.to_bytes().escape_ascii()<'_>)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:182:38
    |
    |
182 |             write!(f, " at byte pos {pos}")?;
    |                                      ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
182 |             write!(f, " at byte pos {pos<'_>}")?;

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:624:17
    |
    |
624 |                 bytes.len(),
    |                 ^^^^^^^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
624 |                 bytes.len()<'_>,

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/c_str.rs:625:17
    |
---
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
325 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/mod.rs:343:10
    |
    |
343 | #[derive(Debug)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
343 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/ffi/mod.rs:418:9
    |
    |
418 |         VaList { inner: self, _marker: PhantomData }
    |         ^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
418 |         VaList<'_, '_> { inner: self, _marker: PhantomData }

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/array_chunks.rs:12:10
   |
   |
12 | #[derive(Debug, Clone)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
12 | #[derive(Debug<'_>, Clone)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/by_ref_sized.rs:11:10
   |
   |
11 | #[derive(Debug)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
11 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/chain.rs:19:17
   |
   |
19 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
19 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/cloned.rs:16:17
   |
   |
16 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
16 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/copied.rs:16:17
   |
   |
16 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
16 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/cycle.rs:10:17
   |
   |
10 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
10 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/enumerate.rs:14:17
   |
   |
14 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
14 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/iter/adapters/flatten.rs:307:17
    |
    |
307 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
307 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/fuse.rs:14:17
   |
   |
14 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
14 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
 --> library/core/src/iter/adapters/intersperse.rs:8:10
  |
  |
8 | #[derive(Debug, Clone)]
  |          ^^^^^ expected lifetime parameter
  |
help: indicate the anonymous lifetime
  |
8 | #[derive(Debug<'_>, Clone)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/peekable.rs:12:17
   |
   |
12 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
12 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/rev.rs:11:17
   |
   |
11 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
11 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/skip.rs:12:17
   |
   |
12 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
12 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/step_by.rs:12:17
   |
   |
12 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
12 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/adapters/take.rs:12:17
   |
   |
12 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
12 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panic.rs:90:72
   |
   |
90 |         $crate::panic!("internal error: entered unreachable code: {}", $crate::format_args!($($t)+))
   |                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
90 |         $crate::panic!("internal error: entered unreachable code: {}", $crate::format_args!($($t)+)<'_>)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/iter/adapters/mod.rs:163:17
    |
    |
163 |     let shunt = GenericShunt { iter, residual: &mut residual };
    |                 ^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
163 |     let shunt = GenericShunt<'_> { iter, residual: &mut residual };

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/sources/once.rs:62:17
   |
   |
62 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
62 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/sources/once_with.rs:69:17
   |
   |
69 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
69 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/sources/repeat.rs:62:17
   |
   |
62 | #[derive(Clone, Debug)]
   |                 ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
62 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/iter/sources/repeat_with.rs:73:23
   |
   |
73 | #[derive(Copy, Clone, Debug)]
   |                       ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
73 | #[derive(Copy, Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/option.rs:515:48
    |
    |
515 | #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    |                                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
515 | #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug<'_>, Hash)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:1158:9
     |
     |
1158 |         Iter { inner: Item { opt: self.as_ref() } }
     |         ^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1158 |         Iter<'_> { inner: Item { opt: self.as_ref() } }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:1179:9
     |
     |
1179 |         IterMut { inner: Item { opt: self.as_mut() } }
     |         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1179 |         IterMut<'_> { inner: Item { opt: self.as_mut() } }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:2039:17
     |
     |
2039 | #[derive(Clone, Debug)]
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2039 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:2078:10
     |
     |
2078 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2078 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:2118:9
     |
     |
2118 |         Iter { inner: self.inner.clone() }
     |         ^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2118 |         Iter<'_> { inner: self.inner.clone() }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:2128:10
     |
     |
2128 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2128 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/option.rs:2168:17
     |
     |
2168 | #[derive(Clone, Debug)]
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2168 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panic/location.rs:31:23
   |
   |
31 | #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
31 | #[derive(Copy, Clone, Debug<'_>, Eq, Hash, Ord, PartialEq, PartialOrd)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panic/location.rs:191:9
    |
    |
191 |         Location { file, line, col }
    |         ^^^^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
191 |         Location<'_> { file, line, col }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panic/location.rs:198:39
    |
    |
198 |         write!(formatter, "{}:{}:{}", self.file, self.line, self.col)
    |                                       ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
198 |         write!(formatter, "{}:{}:{}", self.file<'_>, self.line, self.col)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panic/location.rs:198:50
    |
    |
198 |         write!(formatter, "{}:{}:{}", self.file, self.line, self.col)
    |                                                  ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
198 |         write!(formatter, "{}:{}:{}", self.file, self.line<'_>, self.col)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panic/location.rs:198:61
    |
    |
198 |         write!(formatter, "{}:{}:{}", self.file, self.line, self.col)
    |                                                             ^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
198 |         write!(formatter, "{}:{}:{}", self.file, self.line, self.col<'_>)

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panic/panic_info.rs:29:10
   |
   |
29 | #[derive(Debug)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
29 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panic/panic_info.rs:51:9
   |
   |
51 |         PanicInfo { location, message, payload: &NoPayload, can_unwind }
   |         ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
51 |         PanicInfo<'_> { location, message, payload: &NoPayload, can_unwind }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panic/panic_info.rs:155:41
    |
    |
155 |             write!(formatter, "'{}', ", message)?
    |                                         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
155 |             write!(formatter, "'{}', ", message<'_>)?

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panic/panic_info.rs:157:41
    |
    |
157 |             write!(formatter, "'{}', ", payload)?
    |                                         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
157 |             write!(formatter, "'{}', ", payload<'_>)?

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panicking.rs:62:14
   |
   |
62 |     let pi = PanicInfo::internal_constructor(Some(&fmt), Location::caller(), true);
   |              |
   |              expected lifetime parameter
   |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
   |
62 |     let pi = PanicInfo<'_>::internal_constructor(Some(&fmt), Location::caller(), true);

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panicking.rs:62:58
   |
   |
62 |     let pi = PanicInfo::internal_constructor(Some(&fmt), Location::caller(), true);
   |                                                          |
   |                                                          expected lifetime parameter
   |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
   |
62 |     let pi = PanicInfo::internal_constructor(Some(&fmt), Location<'_>::caller(), true);

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panicking.rs:89:20
   |
   |
89 |     let fmt = fmt::Arguments::new_v1(&pieces, &[]);
   |               -----^^^^^^^^^-------- expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
89 |     let fmt = fmt::Arguments<'_>::new_v1(&pieces, &[]);

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panicking.rs:90:14
   |
   |
90 |     let pi = PanicInfo::internal_constructor(Some(&fmt), Location::caller(), false);
   |              |
   |              expected lifetime parameter
   |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
   |
90 |     let pi = PanicInfo<'_>::internal_constructor(Some(&fmt), Location::caller(), false);

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/panicking.rs:90:58
   |
   |
---
    |             ^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
250 |             op<'_>, left, right,

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/panicking.rs:250:17
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/result.rs:500:48
    |
500 | #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    |                                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
500 | #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug<'_>, Hash)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/result.rs:984:9
    |
    |
984 |         Iter { inner: self.as_ref().ok() }
    |         ^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
984 |         Iter<'_> { inner: self.as_ref().ok() }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/result.rs:1009:9
     |
     |
1009 |         IterMut { inner: self.as_mut().ok() }
     |         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1009 |         IterMut<'_> { inner: self.as_mut().ok() }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/result.rs:1791:14
     |
     |
1791 |     panic!("{msg}: {error:?}")
     |              ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1791 |     panic!("{msg<'_>}: {error:?}")

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/result.rs:1791:21
     |
     |
1791 |     panic!("{msg}: {error:?}")
     |                     ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1791 |     panic!("{msg}: {error<'_>:?}")

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/result.rs:1892:10
     |
     |
1892 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1892 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/result.rs:1934:9
     |
---
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1941 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/result.rs:1987:17
     |
     |
1987 | #[derive(Clone, Debug)]
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1987 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/sync/atomic.rs:212:23
    |
    |
212 | #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
212 | #[derive(Copy, Clone, Debug<'_>, Eq, PartialEq, Hash)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/fmt/builders.rs:26:45
   |
   |
26 |         fmt.wrap_buf(move |buf| slot.insert(PadAdapter { buf, state }))
   |                                             ^^^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
   |
   |
26 |         fmt.wrap_buf(move |buf| slot.insert(PadAdapter<'_, '_> { buf, state }))

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/fmt/builders.rs:90:5
   |
   |
90 |     DebugStruct { fmt, result, has_fields: false }
   |     ^^^^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
   |
   |
90 |     DebugStruct<'_, '_> { fmt, result, has_fields: false }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:131:34
    |
    |
131 |                 let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
    |                                  |
    |                                  expected lifetime parameters
    |
help: indicate the anonymous lifetimes
help: indicate the anonymous lifetimes
    |
131 |                 let mut writer = PadAdapter<'_, '_>::wrap(self.fmt, &mut slot, &mut state);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:182:38
    |
    |
182 |                     let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
    |                                      |
    |                                      expected lifetime parameters
    |
help: indicate the anonymous lifetimes
help: indicate the anonymous lifetimes
    |
182 |                     let mut writer = PadAdapter<'_, '_>::wrap(self.fmt, &mut slot, &mut state);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:280:5
    |
    |
280 |     DebugTuple { fmt, result, fields: 0, empty_name: name.is_empty() }
    |     ^^^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
280 |     DebugTuple<'_, '_> { fmt, result, fields: 0, empty_name: name.is_empty() }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:316:34
    |
    |
316 |                 let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
    |                                  |
    |                                  expected lifetime parameters
    |
help: indicate the anonymous lifetimes
help: indicate the anonymous lifetimes
    |
316 |                 let mut writer = PadAdapter<'_, '_>::wrap(self.fmt, &mut slot, &mut state);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:387:34
    |
    |
387 |                 let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
    |                                  |
    |                                  expected lifetime parameters
    |
help: indicate the anonymous lifetimes
help: indicate the anonymous lifetimes
    |
387 |                 let mut writer = PadAdapter<'_, '_>::wrap(self.fmt, &mut slot, &mut state);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:440:5
    |
    |
440 |     DebugSet { inner: DebugInner { fmt, result, has_fields: false } }
    |     ^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
440 |     DebugSet<'_, '_> { inner: DebugInner { fmt, result, has_fields: false } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:440:23
    |
    |
440 |     DebugSet { inner: DebugInner { fmt, result, has_fields: false } }
    |                       ^^^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
440 |     DebugSet { inner: DebugInner<'_, '_> { fmt, result, has_fields: false } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:570:5
    |
    |
570 |     DebugList { inner: DebugInner { fmt, result, has_fields: false } }
    |     ^^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
570 |     DebugList<'_, '_> { inner: DebugInner { fmt, result, has_fields: false } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:570:24
    |
    |
570 |     DebugList { inner: DebugInner { fmt, result, has_fields: false } }
    |                        ^^^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
570 |     DebugList { inner: DebugInner<'_, '_> { fmt, result, has_fields: false } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:705:5
    |
    |
705 |     DebugMap { fmt, result, has_fields: false, has_key: false, state: Default::default() }
    |     ^^^^^^^^ expected lifetime parameters
help: indicate the anonymous lifetimes
    |
    |
705 |     DebugMap<'_, '_> { fmt, result, has_fields: false, has_key: false, state: Default::default() }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:782:34
    |
    |
782 |                 let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut self.state);
    |                                  |
    |                                  expected lifetime parameters
    |
help: indicate the anonymous lifetimes
help: indicate the anonymous lifetimes
    |
782 |                 let mut writer = PadAdapter<'_, '_>::wrap(self.fmt, &mut slot, &mut self.state);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/builders.rs:838:34
    |
    |
838 |                 let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut self.state);
    |                                  |
    |                                  expected lifetime parameters
    |
help: indicate the anonymous lifetimes
help: indicate the anonymous lifetimes
    |
838 |                 let mut writer = PadAdapter<'_, '_>::wrap(self.fmt, &mut slot, &mut self.state);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/num.rs:138:71
    |
    |
138 |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    |                                                                       ^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
138 |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1<'_>, x),

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/num.rs:138:87
    |
    |
138 |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    |                                                                                       ^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
138 |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x<'_>),

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/num.rs:419:37
    |
    |
293 | / macro_rules! impl_Exp {
294 | |     ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
295 | |         fn $name(
296 | |             mut n: $u,
...   |
419 | |             let formatted = numfmt::Formatted{sign, parts};
    | |                             |
    | |                             expected lifetime parameter
...   |
453 | |     };
453 | |     };
454 | | }
    | |_- in this expansion of `impl_Exp!`
...
465 | /     impl_Exp!(
466 | |         i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
467 | |             as u64 via to_u64 named exp_u64
    | |_____- in this macro invocation
    |
help: indicate the anonymous lifetime
    |
    |
419 |             let formatted = numfmt::Formatted<'_>{sign, parts};

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/num.rs:419:37
    |
    |
293 | / macro_rules! impl_Exp {
294 | |     ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
295 | |         fn $name(
296 | |             mut n: $u,
...   |
419 | |             let formatted = numfmt::Formatted{sign, parts};
    | |                             |
    | |                             expected lifetime parameter
...   |
453 | |     };
453 | |     };
454 | | }
    | |_- in this expansion of `impl_Exp!`
...
479 |   impl_Exp!(i128, u128 as u128 via to_u128 named exp_u128);
    |
help: indicate the anonymous lifetime
    |
    |
419 |             let formatted = numfmt::Formatted<'_>{sign, parts};

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/fmt/mod.rs:25:23
   |
   |
25 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
25 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/fmt/mod.rs:98:23
   |
   |
98 | #[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
98 | #[derive(Copy, Clone, Debug<'_>, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/mod.rs:243:9
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/mod.rs:342:18
    |
342 |         unsafe { ArgumentV1 { formatter: mem::transmute(f), value: mem::transmute(x) } }
    |                  ^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
342 |         unsafe { ArgumentV1<'_> { formatter: mem::transmute(f), value: mem::transmute(x) } }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/mod.rs:358:9
    |
    |
358 |         ArgumentV1::new(x, USIZE_MARKER)
    |         |
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
358 |         ArgumentV1<'_>::new(x, USIZE_MARKER)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/mod.rs:398:9
    |
    |
398 |         Arguments { pieces, fmt: None, args }
    |         ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
398 |         Arguments<'_> { pieces, fmt: None, args }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/fmt/mod.rs:420:9
    |
    |
420 |         Arguments { pieces, fmt: Some(fmt), args }
    |         ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
420 |         Arguments<'_> { pieces, fmt: Some(fmt), args }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/fmt/mod.rs:1196:25
     |
     |
1196 |     let mut formatter = Formatter::new(output);
     |                         ^^^^^^^^^-----
     |                         |
     |                         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1196 |     let mut formatter = Formatter<'_>::new(output);

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/fmt/mod.rs:1300:9
     |
---

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/fmt/mod.rs:2613:38
     |
2613 |         write!(f, "PhantomData<{}>", crate::any::type_name::<T>())
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2613 |         write!(f, "PhantomData<{}>", crate::any::type_name::<T>()<'_>)

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/hash/sip.rs:18:10
   |
   |
18 | #[derive(Debug, Clone, Default)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
18 | #[derive(Debug<'_>, Clone, Default)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/hash/sip.rs:29:10
   |
   |
29 | #[derive(Debug, Clone, Default)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
29 | #[derive(Debug<'_>, Clone, Default)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/hash/sip.rs:48:10
   |
   |
48 | #[derive(Debug, Clone, Default)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
48 | #[derive(Debug<'_>, Clone, Default)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/hash/sip.rs:51:10
   |
   |
51 | #[derive(Debug)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
51 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/hash/sip.rs:62:10
   |
   |
62 | #[derive(Debug, Clone, Copy)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
62 | #[derive(Debug<'_>, Clone, Copy)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/hash/sip.rs:367:10
    |
    |
367 | #[derive(Debug, Clone, Default)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
367 | #[derive(Debug<'_>, Clone, Default)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/hash/sip.rs:384:10
    |
    |
384 | #[derive(Debug, Clone, Default)]
    |          ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
384 | #[derive(Debug<'_>, Clone, Default)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/ascii.rs:79:9
   |
   |
79 |         EscapeAscii { inner: self.iter().flat_map(EscapeByte) }
   |         ^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
79 |         EscapeAscii<'_> { inner: self.iter().flat_map(EscapeByte) }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/ascii.rs:173:10
    |
---

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/index.rs:53:32
   |
53 |     panic!("range start index {index} out of range for slice of length {len}");
   |                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
53 |     panic!("range start index {index<'_>} out of range for slice of length {len}");

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/index.rs:53:73
   |
   |
53 |     panic!("range start index {index} out of range for slice of length {len}");
   |                                                                         ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
53 |     panic!("range start index {index} out of range for slice of length {len<'_>}");

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/index.rs:76:30
   |
   |
76 |     panic!("range end index {index} out of range for slice of length {len}");
   |                              ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
76 |     panic!("range end index {index<'_>} out of range for slice of length {len}");

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/index.rs:76:71
   |
   |
76 |     panic!("range end index {index} out of range for slice of length {len}");
   |                                                                       ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
76 |     panic!("range end index {index} out of range for slice of length {len<'_>}");

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/index.rs:97:36
   |
   |
97 |     panic!("slice index starts at {index} but ends at {end}");
   |                                    ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
97 |     panic!("slice index starts at {index<'_>} but ends at {end}");

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/slice/index.rs:97:56
   |
   |
97 |     panic!("slice index starts at {index} but ends at {end}");
   |                                                        ^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
97 |     panic!("slice index starts at {index} but ends at {end<'_>}");

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/iter.rs:149:9
    |
    |
149 |         Iter { ptr: self.ptr, end: self.end, _marker: self._marker }
    |         ^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
149 |         Iter<'_> { ptr: self.ptr, end: self.end, _marker: self._marker }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/iter.rs:441:9
    |
    |
441 |         Split { v: self.v, pred: self.pred.clone(), finished: self.finished }
    |         ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
441 |         Split<'_> { v: self.v, pred: self.pred.clone(), finished: self.finished }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/iter.rs:574:9
    |
    |
574 |         SplitInclusive { v: self.v, pred: self.pred.clone(), finished: self.finished }
    |         ^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
574 |         SplitInclusive<'_> { v: self.v, pred: self.pred.clone(), finished: self.finished }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/iter.rs:921:23
    |
    |
921 |         Self { inner: Split::new(slice, pred) }
    |                       |
    |                       expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
921 |         Self { inner: Split<'_>::new(slice, pred) }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/iter.rs:945:9
    |
    |
945 |         RSplit { inner: self.inner.clone() }
    |         ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
945 |         RSplit<'_> { inner: self.inner.clone() }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1018:23
     |
     |
1018 |         Self { inner: SplitMut::new(slice, pred) }
     |                       |
     |                       expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1018 |         Self { inner: SplitMut<'_>::new(slice, pred) }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1081:10
     |
     |
1081 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1081 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1295:10
     |
     |
1295 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1295 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1314:9
     |
     |
1314 |         Windows { v: self.v, size: self.size }
     |         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1314 |         Windows<'_> { v: self.v, size: self.size }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1443:10
     |
     |
1443 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1443 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1462:9
     |
     |
1462 |         Chunks { v: self.v, chunk_size: self.chunk_size }
     |         ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1462 |         Chunks<'_> { v: self.v, chunk_size: self.chunk_size }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1626:10
     |
     |
1626 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1626 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1815:10
     |
     |
1815 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1815 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1848:9
     |
     |
1848 |         ChunksExact { v: self.v, rem: self.rem, chunk_size: self.chunk_size }
     |         ^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1848 |         ChunksExact<'_> { v: self.v, rem: self.rem, chunk_size: self.chunk_size }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:1974:10
     |
     |
1974 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1974 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2144:10
     |
     |
2144 | #[derive(Debug, Clone, Copy)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2144 | #[derive(Debug<'_>, Clone, Copy)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2144:17
     |
     |
2144 | #[derive(Debug, Clone, Copy)]
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2144 | #[derive(Debug, Clone<'_>, Copy)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2267:10
     |
     |
2267 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2267 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2296:9
     |
     |
2296 |         ArrayChunks { iter: self.iter.clone(), rem: self.rem }
     |         ^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2296 |         ArrayChunks<'_> { iter: self.iter.clone(), rem: self.rem }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2393:10
     |
     |
2393 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2393 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2507:10
     |
     |
2507 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2507 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2526:9
     |
     |
2526 |         RChunks { v: self.v, chunk_size: self.chunk_size }
     |         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2526 |         RChunks<'_> { v: self.v, chunk_size: self.chunk_size }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2680:10
     |
     |
2680 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2680 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2874:10
     |
     |
2874 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2874 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:2906:9
     |
     |
2906 |         RChunksExact { v: self.v, rem: self.rem, chunk_size: self.chunk_size }
     |         ^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2906 |         RChunksExact<'_> { v: self.v, rem: self.rem, chunk_size: self.chunk_size }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:3036:10
     |
     |
3036 | #[derive(Debug)]
     |          ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
3036 | #[derive(Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:3230:9
     |
     |
3230 |         GroupBy { slice, predicate }
     |         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
3230 |         GroupBy<'_> { slice, predicate }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/iter.rs:3317:9
     |
     |
3317 |         GroupByMut { slice, predicate }
     |         ^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
3317 |         GroupByMut<'_> { slice, predicate }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/sort.rs:898:79
    |
    |
898 |         panic!("partition_at_index index {} greater than length of slice {}", index, v.len());
    |                                                                               ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
898 |         panic!("partition_at_index index {} greater than length of slice {}", index<'_>, v.len());

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/sort.rs:898:86
    |
    |
898 |         panic!("partition_at_index index {} greater than length of slice {}", index, v.len());
    |                                                                                      ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
898 |         panic!("partition_at_index index {} greater than length of slice {}", index, v.len()<'_>);

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/mod.rs:739:9
    |
---
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
758 |         IterMut<'_>::new(self)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/mod.rs:791:9
    |
    |
791 |         Windows::new(self, size)
    |         ^^^^^^^-----
    |         |
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
791 |         Windows<'_>::new(self, size)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/macros/mod.rs:108:114
    |
    |
108 | ...ght_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
108 |                     $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)<'_>));

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/mod.rs:825:9
    |
    |
825 |         Chunks::new(self, chunk_size)
    |         |
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
825 |         Chunks<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/mod.rs:863:9
    |
    |
863 |         ChunksMut::new(self, chunk_size)
    |         |
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
863 |         ChunksMut<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/mod.rs:900:9
    |
    |
900 |         ChunksExact::new(self, chunk_size)
    |         |
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
900 |         ChunksExact<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/slice/mod.rs:942:9
    |
    |
942 |         ChunksExactMut::new(self, chunk_size)
    |         |
    |         expected lifetime parameter
    |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
    |
942 |         ChunksExactMut<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1080:9
     |
     |
1080 |         ArrayChunks::new(self)
     |         ^^^^^^^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1080 |         ArrayChunks<'_>::new(self)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1234:9
     |
     |
1234 |         ArrayChunksMut::new(self)
     |         ^^^^^^^^^^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1234 |         ArrayChunksMut<'_>::new(self)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1266:9
     |
     |
1266 |         ArrayWindows::new(self)
     |         ^^^^^^^^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1266 |         ArrayWindows<'_>::new(self)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1300:9
     |
     |
1300 |         RChunks::new(self, chunk_size)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1300 |         RChunks<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1338:9
     |
     |
1338 |         RChunksMut::new(self, chunk_size)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1338 |         RChunksMut<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1377:9
     |
     |
1377 |         RChunksExact::new(self, chunk_size)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1377 |         RChunksExact<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1420:9
     |
     |
1420 |         RChunksExactMut::new(self, chunk_size)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1420 |         RChunksExactMut<'_>::new(self, chunk_size)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1465:9
     |
     |
1465 |         GroupBy::new(self, pred)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1465 |         GroupBy<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1510:9
     |
     |
1510 |         GroupByMut::new(self, pred)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
1510 |         GroupByMut<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1904:9
     |
     |
1904 |         Split::new(self, pred)
     |         ^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1904 |         Split<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1926:9
     |
     |
1926 |         SplitMut::new(self, pred)
     |         ^^^^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1926 |         SplitMut<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1962:9
     |
     |
1962 |         SplitInclusive::new(self, pred)
     |         ^^^^^^^^^^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1962 |         SplitInclusive<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:1986:9
     |
     |
1986 |         SplitInclusiveMut::new(self, pred)
     |         ^^^^^^^^^^^^^^^^^-----
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1986 |         SplitInclusiveMut<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:2022:9
     |
     |
2022 |         RSplit::new(self, pred)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2022 |         RSplit<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:2048:9
     |
     |
2048 |         RSplitMut::new(self, pred)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2048 |         RSplitMut<'_>::new(self, pred)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:2076:9
     |
     |
2076 |         SplitN::new(self.split(pred), n)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2076 |         SplitN<'_>::new(self.split(pred), n)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:2102:9
     |
     |
2102 |         SplitNMut::new(self.split_mut(pred), n)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2102 |         SplitNMut<'_>::new(self.split_mut(pred), n)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:2131:9
     |
     |
2131 |         RSplitN::new(self.rsplit(pred), n)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2131 |         RSplitN<'_>::new(self.rsplit(pred), n)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:2158:9
     |
     |
2158 |         RSplitNMut::new(self.rsplit_mut(pred), n)
     |         |
     |         expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2158 |         RSplitNMut<'_>::new(self.rsplit_mut(pred), n)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:3252:17
     |
     |
3252 |                 src_len, dst_len,
     |                 ^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
3252 |                 src_len<'_>, dst_len,

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/slice/mod.rs:3252:26
     |
     |
3252 |                 src_len, dst_len,
     |                          ^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
3252 |                 src_len, dst_len<'_>,

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/str/error.rs:45:38
   |
   |
45 | #[derive(Copy, Eq, PartialEq, Clone, Debug)]
   |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
45 | #[derive(Copy, Eq, PartialEq, Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/error.rs:118:17
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/error.rs:121:71
    |
121 |             write!(f, "incomplete utf-8 byte sequence from index {}", self.valid_up_to)
    |                                                                       ^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
121 |             write!(f, "incomplete utf-8 byte sequence from index {}", self.valid_up_to<'_>)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/error.rs:137:10
    |
    |
137 | #[derive(Debug, Clone, PartialEq, Eq)]
    |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
137 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/str/iter.rs:29:10
   |
---
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
125 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/iter.rs:228:17
    |
    |
228 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
228 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/iter.rs:559:14
    |
    |
559 |     with |s| SplitInternal { matcher: s.matcher.clone(), ..*s }
    |              ^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
559 |     with |s| SplitInternal<'_> { matcher: s.matcher.clone(), ..*s }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/iter.rs:852:14
    |
    |
852 |     with |s| SplitNInternal { iter: s.iter.clone(), ..*s }
    |              ^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
852 |     with |s| SplitNInternal<'_> { iter: s.iter.clone(), ..*s }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1093:17
     |
     |
1093 | #[derive(Clone, Debug)]
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1093 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1133:17
     |
     |
1133 | #[derive(Clone, Debug)]
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1133 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1174:10
     |
---
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1174 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1187:10
     |
---
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1187 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1387:10
     |
---
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1438 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1448:10
     |
---
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1448 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/iter.rs:1455:10
     |
---
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1455 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:167:38
    |
    |
167 | #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
167 | #[derive(Copy, Clone, Eq, PartialEq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:351:10
    |
---
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
351 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:545:9
    |
---
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
634 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:646:9
    |
    |
646 |         MultiCharEqSearcher { haystack, char_eq: self.0, char_indices: haystack.char_indices() }
    |         ^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
646 |         MultiCharEqSearcher<'_> { haystack, char_eq: self.0, char_indices: haystack.char_indices() }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:777:17
    |
    |
777 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
777 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:783:17
    |
    |
783 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
783 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:835:17
    |
    |
835 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
835 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:940:9
    |
    |
940 |         StrSearcher::new(haystack, self)
    |         ^^^^^^^^^^^-----
    |         |
    |         expected lifetime parameters
    |
help: indicate the anonymous lifetimes
    |
940 |         StrSearcher<'_, '_>::new(haystack, self)

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:983:10
    |
    |
983 | #[derive(Clone, Debug)]
    |          ^^^^^ expected lifetime parameters
    |
help: indicate the anonymous lifetimes
    |
983 | #[derive(Clone<'_, '_>, Debug)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:983:17
    |
    |
983 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
983 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:992:17
    |
    |
992 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
992 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/pattern.rs:998:17
    |
    |
998 | #[derive(Clone, Debug)]
    |                 ^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
998 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/pattern.rs:1011:13
     |
---
     |                 ^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1204 | #[derive(Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/str/lossy.rs:34:10
   |
   |
34 | #[derive(Clone, Debug, PartialEq, Eq)]
   |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
34 | #[derive(Clone<'_>, Debug, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/str/lossy.rs:34:17
   |
   |
34 | #[derive(Clone, Debug, PartialEq, Eq)]
   |                 ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
34 | #[derive(Clone, Debug<'_>, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/str/lossy.rs:81:22
   |
   |
81 |         for chunk in Utf8Chunks::new(self.0) {
   |                      |
   |                      expected lifetime parameter
   |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
   |
81 |         for chunk in Utf8Chunks<'_>::new(self.0) {

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/lossy.rs:103:40
    |
    |
103 |                 write!(f, "\\x{:02X}", b)?;
    |                                        ^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
103 |                 write!(f, "\\x{:02X}", b<'_>)?;

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/lossy.rs:144:10
    |
---

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/lossy.rs:254:14
    |
254 |         Some(Utf8Chunk {
    |              ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
254 |         Some(Utf8Chunk<'_> {

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:109:29
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
    |                             ^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
109 |         panic!("byte index {oob_index<'_>} is out of bounds of `{s_trunc}`{ellipsis}");

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:109:62
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
    |                                                              ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc<'_>}`{ellipsis}");

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:109:72
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
    |                                                                        ^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
109 |         panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis<'_>}");

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:116:9
    |
---
    |         ^^^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
118 |         s_trunc<'_>,

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:119:9
    |
    |
119 |         ellipsis
    |         ^^^^^^^^ expected lifetime parameter
    |
help: indicate the anonymous lifetime
    |
119 |         ellipsis<'_>

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:131:9
    |
    |
131 |         index, ch, char_range, s_trunc, ellipsis
    |         ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
131 |         index<'_>, ch, char_range, s_trunc, ellipsis

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:131:16
    |
    |
131 |         index, ch, char_range, s_trunc, ellipsis
    |                ^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
131 |         index, ch<'_>, char_range, s_trunc, ellipsis

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:131:20
    |
    |
131 |         index, ch, char_range, s_trunc, ellipsis
    |                    ^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
131 |         index, ch, char_range<'_>, s_trunc, ellipsis

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:131:32
    |
    |
131 |         index, ch, char_range, s_trunc, ellipsis
    |                                ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
131 |         index, ch, char_range, s_trunc<'_>, ellipsis

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:131:41
    |
    |
131 |         index, ch, char_range, s_trunc, ellipsis
    |                                         ^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
131 |         index, ch, char_range, s_trunc, ellipsis<'_>

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:783:9
    |
    |
783 |         Chars { iter: self.as_bytes().iter() }
    |         ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
783 |         Chars<'_> { iter: self.as_bytes().iter() }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:840:9
    |
    |
840 |         CharIndices { front_offset: 0, iter: self.chars() }
    |         ^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
840 |         CharIndices<'_> { front_offset: 0, iter: self.chars() }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:911:9
    |
    |
911 |         SplitWhitespace { inner: self.split(IsWhitespace).filter(IsNotEmpty) }
    |         ^^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
911 |         SplitWhitespace<'_> { inner: self.split(IsWhitespace).filter(IsNotEmpty) }

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/str/mod.rs:956:9
    |
---

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:1030:9
     |
1030 |         EncodeUtf16 { chars: self.chars(), extra: 0 }
     |         ^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1030 |         EncodeUtf16<'_> { chars: self.chars(), extra: 0 }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:1328:15
     |
     |
1328 |         Split(SplitInternal {
     |               ^^^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1328 |         Split(SplitInternal<'_> {

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:1368:24
     |
     |
1368 |         SplitInclusive(SplitInternal {
     |                        ^^^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1368 |         SplitInclusive(SplitInternal<'_> {

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:1474:25
     |
     |
1474 |         SplitTerminator(SplitInternal { allow_trailing_empty: false, ..self.split(pat).0 })
     |                         ^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1474 |         SplitTerminator(SplitInternal<'_> { allow_trailing_empty: false, ..self.split(pat).0 })

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:1575:16
     |
     |
1575 |         SplitN(SplitNInternal { iter: self.split(pat).0, count: n })
     |                ^^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1575 |         SplitN(SplitNInternal<'_> { iter: self.split(pat).0, count: n })

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:2475:9
     |
     |
2475 |         EscapeDebug {
     |         ^^^^^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
2475 |         EscapeDebug<'_> {

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:2520:9
     |
     |
2520 |         EscapeDefault { inner: self.chars().flat_map(CharEscapeDefault) }
     |         ^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2520 |         EscapeDefault<'_> { inner: self.chars().flat_map(CharEscapeDefault) }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/str/mod.rs:2558:9
     |
     |
2558 |         EscapeUnicode { inner: self.chars().flat_map(CharEscapeUnicode) }
     |         ^^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
2558 |         EscapeUnicode<'_> { inner: self.chars().flat_map(CharEscapeUnicode) }

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1124:39
     |
     |
1124 |                     write!(f, "{}{}", prefix, integer_part)?;
     |                                       ^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1124 |                     write!(f, "{}{}", prefix<'_>, integer_part)?;

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1124:47
     |
     |
1124 |                     write!(f, "{}{}", prefix, integer_part)?;
     |                                               ^^^^^^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1124 |                     write!(f, "{}{}", prefix, integer_part<'_>)?;

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1127:57
     |
     |
1127 |                     write!(f, "{}18446744073709551616", prefix)?;
     |                                                         ^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1127 |                     write!(f, "{}18446744073709551616", prefix<'_>)?;

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1138:47
     |
     |
1138 |                     write!(f, ".{:0<width$}", s, width = w)?;
     |                                               ^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1138 |                     write!(f, ".{:0<width$}", s<'_>, width = w)?;

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1138:58
     |
     |
1138 |                     write!(f, ".{:0<width$}", s, width = w)?;
     |                                                          ^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1138 |                     write!(f, ".{:0<width$}", s, width = w<'_>)?;

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1141:33
     |
     |
1141 |                 write!(f, "{}", postfix)
     |                                 ^^^^^^^ expected lifetime parameter
     |
help: indicate the anonymous lifetime
     |
1141 |                 write!(f, "{}", postfix<'_>)

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1235:10
     |
     |
1235 | #[derive(Debug, Clone, PartialEq, Eq)]
     |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1235 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
    --> library/core/src/time.rs:1261:10
     |
     |
1261 | #[derive(Debug, Clone, PartialEq, Eq)]
     |          ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
     |
     |
1261 | #[derive(Debug<'_>, Clone, PartialEq, Eq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/future/ready.rs:10:10
   |
   |
10 | #[derive(Debug, Clone)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
10 | #[derive(Debug<'_>, Clone)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/future/mod.rs:53:10
   |
   |
53 | #[derive(Debug, Copy, Clone)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
53 | #[derive(Debug<'_>, Copy, Clone)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/task/poll.rs:11:23
   |
   |
11 | #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
11 | #[derive(Copy, Clone, Debug<'_>, Eq, PartialEq, Ord, PartialOrd, Hash)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/task/wake.rs:13:21
   |
   |
13 | #[derive(PartialEq, Debug)]
   |                     ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
13 | #[derive(PartialEq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/task/wake.rs:81:34
   |
   |
81 | #[derive(PartialEq, Copy, Clone, Debug)]
   |                                  ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
81 | #[derive(PartialEq, Copy, Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/task/wake.rs:193:9
    |
    |
193 |         Context { waker, _marker: PhantomData }
    |         ^^^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
193 |         Context<'_> { waker, _marker: PhantomData }

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/alloc/layout.rs:36:23
   |
   |
36 | #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
36 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq, Hash)]

error: hidden lifetime parameters in types are deprecated
   --> library/core/src/alloc/layout.rs:465:32
    |
    |
465 | #[derive(Clone, PartialEq, Eq, Debug)]
    |                                ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
    |
    |
465 | #[derive(Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/alloc/mod.rs:33:38
   |
   |
33 | #[derive(Copy, Clone, PartialEq, Eq, Debug)]
   |                                      ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
33 | #[derive(Copy, Clone, PartialEq, Eq, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
 --> library/core/src/../../stdarch/crates/core_arch/src/simd.rs:8:31
  |
  |
8 |         #[derive(Copy, Clone, Debug, PartialEq)]
  |                               ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
  |
  |
8 |         #[derive(Copy, Clone, Debug<'_>, PartialEq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/../../stdarch/crates/core_arch/src/simd.rs:41:31
   |
   |
41 |         #[derive(Copy, Clone, Debug, PartialEq)]
   |                               ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
41 |         #[derive(Copy, Clone, Debug<'_>, PartialEq)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:31
   |
   |
88 |         #[derive(Copy, Clone, Debug)]
   |                               ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
88 |         #[derive(Copy, Clone, Debug<'_>)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/cpuid.rs:11:23
   |
   |
11 | #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
11 | #[derive(Copy, Clone, Debug<'_>, Eq, Ord, PartialEq, PartialOrd)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/../../portable-simd/crates/core_simd/src/swizzle.rs:76:23
   |
   |
76 | #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
   |                       ^^^^^ expected lifetime parameter
help: indicate the anonymous lifetime
   |
   |
76 | #[derive(Copy, Clone, Debug<'_>, PartialEq, Eq, PartialOrd, Ord, Hash)]

error: hidden lifetime parameters in types are deprecated
 --> library/core/src/../../portable-simd/crates/core_simd/src/vector/ptr.rs:6:10
  |
  |
6 | #[derive(Debug, Copy, Clone)]
  |          ^^^^^ expected lifetime parameter
  |
help: indicate the anonymous lifetime
  |
6 | #[derive(Debug<'_>, Copy, Clone)]

error: hidden lifetime parameters in types are deprecated
  --> library/core/src/../../portable-simd/crates/core_simd/src/vector/ptr.rs:30:10
   |
   |
30 | #[derive(Debug, Copy, Clone)]
   |          ^^^^^ expected lifetime parameter
   |
help: indicate the anonymous lifetime
   |
30 | #[derive(Debug<'_>, Copy, Clone)]

error: could not compile `core` due to 619 previous errors
Build completed unsuccessfully in 0:04:05
