
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]   --> tools/miri/src/lib.rs:80:33
[01:08:04]    |
[01:08:04] 80 |         Scalar::Bits { bits: 0, defined: 128 }
[01:08:04]    |                                 ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]   --> tools/miri/src/lib.rs:84:48
[01:08:04]    |
[01:08:04] 84 |         Scalar::Bits { bits: i as u32 as u128, defined: 32 }
[01:08:04]    |                                                ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]   --> tools/miri/src/lib.rs:88:33
[01:08:04]    |
[01:08:04] 88 |         Scalar::Bits { bits: i, defined: 128 }
[01:08:04]    |                                 ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]   --> tools/miri/src/lib.rs:92:41
[01:08:04]    |
[01:08:04] 92 |         Scalar::Bits { bits: i as u128, defined: 128 }
[01:08:04]    |                                         ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]   --> tools/miri/src/lib.rs:96:41
[01:08:04]    |
[01:08:04] 96 |         Scalar::Bits { bits: i as u128, defined: ptr_size.bits() as u8 }
[01:08:04]    |                                         ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]    --> tools/miri/src/lib.rs:100:49
[01:08:04]     |
[01:08:04] 100 |         Scalar::Bits { bits: i as i128 as u128, defined: ptr_size.bits() as u8 }
[01:08:04]     |                                                 ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]    --> tools/miri/src/lib.rs:104:51
[01:08:04]     |
[01:08:04] 104 |         Scalar::Bits { bits: f.to_bits() as u128, defined: 32 }
[01:08:04]     |                                                   ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:04]    --> tools/miri/src/lib.rs:108:51
[01:08:04]     |
[01:08:04] 108 |         Scalar::Bits { bits: f.to_bits() as u128, defined: 64 }
[01:08:04]     |                                                   ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0026]: variant `rustc::mir::interpret::Scalar::Bits` does not have a field named `defined`
[01:08:04]    --> tools/miri/src/lib.rs:119:34
[01:08:04]     |
[01:08:04] 119 |             Scalar::Bits { bits, defined } => {
[01:08:04]     |                                  ^^^^^^^ variant `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:04] error[E0027]: pattern does not mention field `size`
[01:08:04]    --> tools/miri/src/lib.rs:119:13
[01:08:04]     |
[01:08:04] 119 |             Scalar::Bits { bits, defined } => {
[01:08:04]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `size`
[01:08:04]
[01:08:04] error[E0026]: variant `rustc::mir::interpret::Scalar::Bits` does not have a field named `defined`
[01:08:04]    --> tools/miri/src/lib.rs:132:28
[01:08:04]     |
[01:08:04] 132 |             Scalar::Bits { defined: 0, .. } => err!(ReadUndefBytes),
[01:08:04]     |                            ^^^^^^^^^^ variant `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:04]
[01:08:05] error[E0308]: mismatched types
[01:08:05]    --> tools/miri/src/lib.rs:205:38
[01:08:05]     |
[01:08:05] 205 |                 value: Value::Scalar(Scalar::Ptr(main_ptr)),
[01:08:05]     |                                      ^^^^^^^^^^^^^^^^^^^^^
[01:08:05]     |                                      |
[01:08:05]     |                                      expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:05]     |                                      help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(Scalar::Ptr(main_ptr))`
[01:08:05]     |
[01:08:05]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:05]                found type `rustc::mir::interpret::Scalar`
[01:08:05]
[01:08:05] error[E0061]: this function takes 6 parameters but 5 parameters were supplied
[01:08:05]    --> tools/miri/src/lib.rs:224:20
[01:08:05]     |
[01:08:05] 224 |         ecx.memory.write_scalar(foo_ptr.into(), ptr_align, Scalar::Ptr(foo), ptr_size, false)?;
[01:08:05]     |                    ^^^^^^^^^^^^ expected 6 parameters
[01:08:05]
[01:08:05] error[E0308]: mismatched types
[01:08:05]    --> tools/miri/src/lib.rs:234:36
[01:08:05]     |
[01:08:05] 234 |             Place::from_scalar_ptr(Scalar::from_u128(1), ty::layout::Align::from_bytes(1, 1).unwrap()),
[01:08:05]     |                                    ^^^^^^^^^^^^^^^^^^^^
[01:08:05]     |                                    |
[01:08:05]     |                                    expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:05]     |                                    help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::from_u128(1))`
[01:08:05]     |
[01:08:05]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:05]                found type `rustc::mir::interpret::Scalar`
[01:08:05]
[01:08:05] error[E0308]: mismatched types
[01:08:05]    --> tools/miri/src/lib.rs:300:28
[01:08:05]     |
[01:08:05] 300 |                     if let Some(local) = local {
[01:08:05]     |                            ^^^^^^^^^^^ expected enum `rustc_mir::interpret::eval_context::LocalValue`, found enum `std::option::Option`
[01:08:05]     |
[01:08:05]     = note: expected type `rustc_mir::interpret::eval_context::LocalValue`
[01:08:05]                found type `std::option::Option<_>`
[01:08:05]
[01:08:05] error[E0308]: mismatched types
[01:08:05]    --> tools/miri/src/lib.rs:530:38
[01:08:05]     |
[01:08:05] 530 |                   value: Value::Scalar(Scalar::from_u128(match layout.size.bytes() {
[01:08:05]     |  ______________________________________^
[01:08:05] 531 | |                     0 => 1 as u128,
[01:08:05] 532 | |                     size => size as u128,
[01:08:05] 533 | |                 })),
[01:08:05]     | |__________________^ expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:05]     |
[01:08:05]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:05]                found type `rustc::mir::interpret::Scalar`
[01:08:05] help: try using a variant of the expected type
[01:08:05]     |
[01:08:05] 530 |                 value: Value::Scalar(rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::from_u128(match layout.size.bytes() {
[01:08:05] 531 |                         0 => 1 as u128,
[01:08:05] 532 |                         size => size as u128,
[01:08:05] 533 |                     }))),
[01:08:05]     |
[01:08:05]
[01:08:05] error[E0308]: mismatched types
[01:08:05]    --> tools/miri/src/lib.rs:543:38
[01:08:05]     |
[01:08:05] 543 |                 value: Value::Scalar(Scalar::from_u128(layout.align.abi().into())),
[01:08:05]     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:05]     |                                      |
[01:08:05]     |                                      expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:05]     |                                      help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::from_u128(layout.align.abi().into()))`
[01:08:05]     |
[01:08:05]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:05]                found type `rustc::mir::interpret::Scalar`
[01:08:05]
[01:08:05] error[E0599]: no method named `is_null` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:05]    --> tools/miri/src/fn_call.rs:205:25
[01:08:05]     |
[01:08:05] 205 |                 if !ptr.is_null()? {
[01:08:05]     |                         ^^^^^^^
[01:08:05]     |
[01:08:05]     = help: items from traits can only be used if the trait is implemented and in scope
[01:08:05]     = note: the following trait defines an item `is_null`, perhaps you need to implement it:
[01:08:05]             candidate #1: `ScalarExt`
[01:08:05]
[01:08:05] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:05]    --> tools/miri/src/fn_call.rs:207:29
[01:08:05]     |
[01:08:05] 207 |                         ptr.to_ptr()?,
[01:08:05]     |                             ^^^^^^
[01:08:05]
[01:08:05] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:05]    --> tools/miri/src/fn_call.rs:244:57
[01:08:05]     |
[01:08:05] 244 |                 let ptr = self.into_ptr(args[0].value)?.to_ptr()?;
[01:08:05]     |                                                         ^^^^^^
[01:08:05]
[01:08:05] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:05]    --> tools/miri/src/fn_call.rs:260:57
[01:08:05]     |
[01:08:05] 260 |                 let ptr = self.into_ptr(args[0].value)?.to_ptr()?;
[01:08:05]     |                                                         ^^^^^^
[01:08:05]
[01:08:05] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:05]    --> tools/miri/src/fn_call.rs:303:60
[01:08:05]     |
[01:08:05] 303 |                 let symbol = self.into_ptr(args[1].value)?.to_ptr()?;
[01:08:05]     |                                                            ^^^^^^
[01:08:05]
[01:08:05] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:05]    --> tools/miri/src/fn_call.rs:317:55
[01:08:05]     |
[01:08:05] 317 |                 let f = self.into_ptr(args[0].value)?.to_ptr()?;
[01:08:05]     |                                                       ^^^^^^
[01:08:05]
[01:08:05] error[E0308]: mismatched types
[01:08:05]    --> tools/miri/src/fn_call.rs:341:42
[01:08:05]     |
[01:08:05] 341 |                 self.write_ptr(arg_dest, data, u8_ptr_ty)?;
[01:08:05]     |                                          ^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:05]     |
[01:08:05]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:05]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:05]
[01:08:06] error[E0308]: mismatched types
[01:08:06]    --> tools/miri/src/fn_call.rs:362:61
[01:08:06]     |
[01:08:06] 362 |                     let left_bytes = self.memory.read_bytes(left, n)?;
[01:08:06]     |                                                             ^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]     |
[01:08:06]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:06]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]
[01:08:06] error[E0308]: mismatched types
[01:08:06]    --> tools/miri/src/fn_call.rs:363:62
[01:08:06]     |
[01:08:06] 363 |                     let right_bytes = self.memory.read_bytes(right, n)?;
[01:08:06]     |                                                              ^^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]     |
[01:08:06]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:06]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]
[01:08:06] error[E0308]: mismatched types
[01:08:06]    --> tools/miri/src/fn_call.rs:384:59
[01:08:06]     |
[01:08:06] 384 |                 if let Some(idx) = self.memory.read_bytes(ptr, Size::from_bytes(num))?.iter().rev().position(
[01:08:06]     |                                                           ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]     |
[01:08:06]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:06]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]
[01:08:06] error[E0308]: mismatched types
[01:08:06]    --> tools/miri/src/fn_call.rs:389:42
[01:08:06]     |
[01:08:06] 389 |                     self.write_ptr(dest, new_ptr, dest_ty)?;
[01:08:06]     |                                          ^^^^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]     |
[01:08:06]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:06]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]
[01:08:06] error[E0308]: mismatched types
[01:08:06]    --> tools/miri/src/fn_call.rs:399:59
[01:08:06]     |
[01:08:06] 399 |                 if let Some(idx) = self.memory.read_bytes(ptr, Size::from_bytes(num))?.iter().position(
[01:08:06]     |                                                           ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]     |
[01:08:06]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:06]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]
[01:08:06] error[E0308]: mismatched types
[01:08:06]    --> tools/miri/src/fn_call.rs:404:42
[01:08:06]     |
[01:08:06] 404 |                     self.write_ptr(dest, new_ptr, dest_ty)?;
[01:08:06]     |                                          ^^^^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]     |
[01:08:06]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:06]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:06]
[01:08:06] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:06]    --> tools/miri/src/fn_call.rs:412:66
[01:08:06]     |
[01:08:06] 412 |                     let name_ptr = self.into_ptr(args[0].value)?.to_ptr()?;
[01:08:06]     |                                                                  ^^^^^^
[01:08:06]
[01:08:06] error[E0599]: no method named `is_null` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:06]    --> tools/miri/src/fn_call.rs:426:34
[01:08:06]     |
[01:08:06] 426 |                     if !name_ptr.is_null()? {
[01:08:06]     |                                  ^^^^^^^
[01:08:06]     |
[01:08:06]     = help: items from traits can only be used if the trait is implemented and in scope
[01:08:06]     = note: the following trait defines an item `is_null`, perhaps you need to implement it:
[01:08:06]             candidate #1: `ScalarExt`
[01:08:06]
[01:08:06] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:06]    --> tools/miri/src/fn_call.rs:427:68
[01:08:06]     |
[01:08:06] 427 |                         let name = self.memory.read_c_str(name_ptr.to_ptr()?)?;
[01:08:06]     |                                                                    ^^^^^^
[01:08:06]
[01:08:06] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:06]    --> tools/miri/src/fn_call.rs:447:67
[01:08:06]     |
[01:08:06] 447 |                     let value_ptr = self.into_ptr(args[1].value)?.to_ptr()?;
[01:08:06]     |                                                                   ^^^^^^
[01:08:06]
[01:08:06] error[E0599]: no method named `is_null` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:06]    --> tools/miri/src/fn_call.rs:449:34
[01:08:06]     |
[01:08:06] 449 |                     if !name_ptr.is_null()? {
[01:08:06]     |                                  ^^^^^^^
[01:08:06]     |
[01:08:06]     = help: items from traits can only be used if the trait is implemented and in scope
[01:08:06]     = note: the following trait defines an item `is_null`, perhaps you need to implement it:
[01:08:06]             candidate #1: `ScalarExt`
[01:08:06]
[01:08:06] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:06]    --> tools/miri/src/fn_call.rs:450:68
[01:08:06]     |
[01:08:06] 450 |                         let name = self.memory.read_c_str(name_ptr.to_ptr()?)?;
[01:08:06]     |                                                                    ^^^^^^
[01:08:06]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:488:59
[01:08:07]     |
[01:08:07] 488 |                     let buf_cont = self.memory.read_bytes(buf, Size::from_bytes(n))?;
[01:08:07]     |                                                           ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:07]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]
[01:08:07] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:07]    --> tools/miri/src/fn_call.rs:511:57
[01:08:07]     |
[01:08:07] 511 |                 let ptr = self.into_ptr(args[0].value)?.to_ptr()?;
[01:08:07]     |                                                         ^^^^^^
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:562:21
[01:08:07]     |
[01:08:07] 562 |                     Scalar::Ptr(dtor_ptr) => Some(self.memory.get_fn(dtor_ptr)?),
[01:08:07]     |                     ^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]                found type `rustc::mir::interpret::Scalar`
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:563:21
[01:08:07]     |
[01:08:07] 563 |                     Scalar::Bits { defined: 0, .. } => return err!(ReadUndefBytes),
[01:08:07]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]                found type `rustc::mir::interpret::Scalar`
[01:08:07]
[01:08:07] error[E0026]: variant `rustc::mir::interpret::Scalar::Bits` does not have a field named `defined`
[01:08:07]    --> tools/miri/src/fn_call.rs:563:36
[01:08:07]     |
[01:08:07] 563 |                     Scalar::Bits { defined: 0, .. } => return err!(ReadUndefBytes),
[01:08:07]     |                                    ^^^^^^^^^^ variant `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:564:21
[01:08:07]     |
[01:08:07] 564 |                     Scalar::Bits { bits: 0, .. } => None,
[01:08:07]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]                found type `rustc::mir::interpret::Scalar`
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:565:21
[01:08:07]     |
[01:08:07] 565 |                     Scalar::Bits { .. } => return err!(ReadBytesAsPointer),
[01:08:07]     |                     ^^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]                found type `rustc::mir::interpret::Scalar`
[01:08:07]
[01:08:07] error[E0061]: this function takes 6 parameters but 5 parameters were supplied
[01:08:07]    --> tools/miri/src/fn_call.rs:578:29
[01:08:07]     |
[01:08:07] 578 |                 self.memory.write_scalar(
[01:08:07]     |                             ^^^^^^^^^^^^ expected 6 parameters
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:603:44
[01:08:07]     |
[01:08:07] 603 |                 self.memory.store_tls(key, new_ptr)?;
[01:08:07]     |                                            ^^^^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:07]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:622:38
[01:08:07]     |
[01:08:07] 622 |                 self.write_ptr(dest, addr, dest_ty)?;
[01:08:07]     |                                      ^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:07]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]    --> tools/miri/src/fn_call.rs:668:44
[01:08:07]     |
[01:08:07] 668 |                 self.memory.store_tls(key, new_ptr)?;
[01:08:07]     |                                            ^^^^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]     |
[01:08:07]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:07]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]
[01:08:07] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:07]    --> tools/miri/src/operator.rs:119:83
[01:08:07]     |
[01:08:07] 119 |                                 Scalar::Bits { bits: left.offset.bytes() as u128, defined: self.memory.pointer_size().bits() as u8 },
[01:08:07]     |                                                                                   ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:07]
[01:08:07] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:07]    --> tools/miri/src/operator.rs:121:84
[01:08:07]     |
[01:08:07] 121 |                                 Scalar::Bits { bits: right.offset.bytes() as u128, defined: self.memory.pointer_size().bits() as u8 },
[01:08:07]     |                                                                                    ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:07]
[01:08:07] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:07]    --> tools/miri/src/operator.rs:190:82
[01:08:07]     |
[01:08:07] 190 |                     (Scalar::Bits { bits: (left.offset.bytes() & right) as u128, defined: 128 }, false)
[01:08:07]     |                                                                                  ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]   --> tools/miri/src/intrinsic.rs:69:63
[01:08:07]    |
[01:08:07] 69 |                 let result_ptr = self.wrapping_pointer_offset(ptr, substs.type_at(0), offset)?;
[01:08:07]    |                                                               ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]    |
[01:08:07]    = note: expected type `rustc::mir::interpret::Scalar`
[01:08:07]               found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]
[01:08:07] error[E0308]: mismatched types
[01:08:07]   --> tools/miri/src/intrinsic.rs:88:41
[01:08:07]    |
[01:08:07] 88 |                     value: Value::ByRef(ptr, align),
[01:08:07]    |                                         ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]    |
[01:08:07]    = note: expected type `rustc::mir::interpret::Scalar`
[01:08:07]               found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:07]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:101:56
[01:08:08]     |
[01:08:08] 101 |                 self.write_value_to_ptr(args[1].value, dest, align, ty)?;
[01:08:08]     |                                                        ^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:08]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:113:43
[01:08:08]     |
[01:08:08] 113 |                 let old = self.read_value(ptr, align, ty)?;
[01:08:08]     |                                           ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:08]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:133:43
[01:08:08]     |
[01:08:08] 133 |                 let old = self.read_value(ptr, align, ty)?;
[01:08:08]     |                                           ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:08]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:139:63
[01:08:08]     |
[01:08:08] 139 |                 let (val, _) = self.binary_op(mir::BinOp::Eq, old, ty, expect_old, ty)?;
[01:08:08]     |                                                               ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:08]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:141:51
[01:08:08]     |
[01:08:08] 141 |                     value: Value::ScalarPair(old, val),
[01:08:08]     |                                                   ^^^
[01:08:08]     |                                                   |
[01:08:08]     |                                                   expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:08]     |                                                   help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(val)`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]                found type `rustc::mir::interpret::Scalar`
[01:08:08]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:181:43
[01:08:08]     |
[01:08:08] 181 |                 let old = self.read_value(ptr, align, ty)?;
[01:08:08]     |                                           ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:08]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]
[01:08:08] error[E0308]: mismatched types
[01:08:08]    --> tools/miri/src/intrinsic.rs:199:51
[01:08:08]     |
[01:08:08] 199 |                 let (val, _) = self.binary_op(op, old, ty, change, ty)?;
[01:08:08]     |                                                   ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]     |
[01:08:08]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:08]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:08]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:218:25
[01:08:09]     |
[01:08:09] 218 |                         src,
[01:08:09]     |                         ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:09]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:220:25
[01:08:09]     |
[01:08:09] 220 |                         dest,
[01:08:09]     |                         ^^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:09]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:342:39
[01:08:09]     |
[01:08:09] 342 |                         Value::Scalar(Scalar::Bits { defined: 0, .. }) => {
[01:08:09]     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]                found type `rustc::mir::interpret::Scalar`
[01:08:09]
[01:08:09] error[E0026]: variant `rustc::mir::interpret::Scalar::Bits` does not have a field named `defined`
[01:08:09]    --> tools/miri/src/intrinsic.rs:342:54
[01:08:09]     |
[01:08:09] 342 |                         Value::Scalar(Scalar::Bits { defined: 0, .. }) => {
[01:08:09]     |                                                      ^^^^^^^^^^ variant `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:09]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:344:77
[01:08:09]     |
[01:08:09] 344 |                                 ty::layout::Abi::Scalar(_) => Value::Scalar(Scalar::null()),
[01:08:09]     |                                                                             ^^^^^^^^^^^^^^
[01:08:09]     |                                                                             |
[01:08:09]     |                                                                             expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:09]     |                                                                             help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::null())`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]                found type `rustc::mir::interpret::Scalar`
[01:08:09]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:354:59
[01:08:09]     |
[01:08:09] 354 |                         Value::Scalar(_) => Value::Scalar(Scalar::null()),
[01:08:09]     |                                                           ^^^^^^^^^^^^^^
[01:08:09]     |                                                           |
[01:08:09]     |                                                           expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:09]     |                                                           help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::null())`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]                found type `rustc::mir::interpret::Scalar`
[01:08:09]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:356:47
[01:08:09]     |
[01:08:09] 356 |                             Value::ScalarPair(Scalar::null(), Scalar::null())
[01:08:09]     |                                               ^^^^^^^^^^^^^^
[01:08:09]     |                                               |
[01:08:09]     |                                               expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:09]     |                                               help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::null())`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]                found type `rustc::mir::interpret::Scalar`
[01:08:09]
[01:08:09] error[E0308]: mismatched types
[01:08:09]    --> tools/miri/src/intrinsic.rs:356:63
[01:08:09]     |
[01:08:09] 356 |                             Value::ScalarPair(Scalar::null(), Scalar::null())
[01:08:09]     |                                                               ^^^^^^^^^^^^^^
[01:08:09]     |                                                               |
[01:08:09]     |                                                               expected enum `rustc::mir::interpret::ScalarMaybeUndef`, found enum `rustc::mir::interpret::Scalar`
[01:08:09]     |                                                               help: try using a variant of the expected type: `rustc::mir::interpret::ScalarMaybeUndef::Scalar(<Scalar>::null())`
[01:08:09]     |
[01:08:09]     = note: expected type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:09]                found type `rustc::mir::interpret::Scalar`
[01:08:09]
[01:08:09] error[E0599]: no method named `modify_local` found for type `&mut rustc_mir::interpret::EvalContext<'a, 'mir, 'tcx, Evaluator<'tcx>>` in the current scope
[01:08:09]    --> tools/miri/src/intrinsic.rs:362:59
[01:08:09]     |
[01:08:09] 362 |                     Place::Local { frame, local } => self.modify_local(frame, local, init)?,
[01:08:09]     |                                                           ^^^^^^^^^^^^
[01:08:09]
[01:08:10] error[E0308]: mismatched types
[01:08:10]    --> tools/miri/src/intrinsic.rs:367:51
[01:08:10]     |
[01:08:10] 367 |                     } => self.memory.write_repeat(ptr, 0, size)?,
[01:08:10]     |                                                   ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]     |
[01:08:10]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:10]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]
[01:08:10] error[E0308]: mismatched types
[01:08:10]    --> tools/miri/src/intrinsic.rs:393:56
[01:08:10]     |
[01:08:10] 393 |                 self.write_value_to_ptr(args[1].value, ptr, align, ty)?;
[01:08:10]     |                                                        ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]     |
[01:08:10]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:10]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]
[01:08:10] error[E0308]: mismatched types
[01:08:10]    --> tools/miri/src/intrinsic.rs:410:54
[01:08:10]     |
[01:08:10] 410 |                 let result_ptr = self.pointer_offset(ptr, substs.type_at(0), offset)?;
[01:08:10]     |                                                      ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]     |
[01:08:10]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:10]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]
[01:08:10] error[E0559]: variant `rustc::mir::interpret::Scalar::Bits` has no field named `defined`
[01:08:10]    --> tools/miri/src/intrinsic.rs:554:73
[01:08:10]     |
[01:08:10] 554 |                 self.write_scalar(dest, Scalar::Bits { bits: n as u128, defined: 64 }, dest_layout.ty)?;
[01:08:10]     |                                                                         ^^^^^^^ `rustc::mir::interpret::Scalar::Bits` does not have this field
[01:08:10]
[01:08:10] error[E0599]: no variant named `undef` found for type `rustc::mir::interpret::Scalar` in the current scope
[01:08:10]    --> tools/miri/src/intrinsic.rs:638:43
[01:08:10]     |
[01:08:10] 638 |                     _ => Ok(Value::Scalar(Scalar::undef())),
[01:08:10]     |                                           ^^^^^^^^^^^^^ variant not found in `rustc::mir::interpret::Scalar`
[01:08:10]
[01:08:10] error[E0599]: no method named `modify_local` found for type `&mut rustc_mir::interpret::EvalContext<'a, 'mir, 'tcx, Evaluator<'tcx>>` in the current scope
[01:08:10]    --> tools/miri/src/intrinsic.rs:641:59
[01:08:10]     |
[01:08:10] 641 |                     Place::Local { frame, local } => self.modify_local(frame, local, uninit)?,
[01:08:10]     |                                                           ^^^^^^^^^^^^
[01:08:10]
[01:08:10] error[E0308]: mismatched types
[01:08:10]    --> tools/miri/src/intrinsic.rs:646:55
[01:08:10]     |
[01:08:10] 646 |                     } => self.memory.mark_definedness(ptr, size, false)?,
[01:08:10]     |                                                       ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]     |
[01:08:10]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:10]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]
[01:08:10] error[E0308]: mismatched types
[01:08:10]    --> tools/miri/src/intrinsic.rs:662:45
[01:08:10]     |
[01:08:10] 662 |                     self.memory.check_align(ptr, ty_layout.align)?;
[01:08:10]     |                                             ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]     |
[01:08:10]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:10]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]
[01:08:10] error[E0308]: mismatched types
[01:08:10]    --> tools/miri/src/intrinsic.rs:663:46
[01:08:10]     |
[01:08:10] 663 |                     self.memory.write_repeat(ptr, val_byte, ty_layout.size * count)?;
[01:08:10]     |                                              ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]     |
[01:08:10]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:10]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:10]
[01:08:11] error[E0308]: mismatched types
[01:08:11]   --> tools/miri/src/helpers.rs:54:9
[01:08:11]    |
[01:08:11] 50 |     ) -> EvalResult<'tcx, Scalar> {
[01:08:11]    |          ------------------------ expected `std::result::Result<rustc::mir::interpret::Scalar, rustc::mir::interpret::EvalError<'tcx>>` because of return type
[01:08:11] ...
[01:08:11] 54 |         ptr.ptr_wrapping_signed_offset(offset, self)
[01:08:11]    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found enum `rustc::mir::interpret::Scalar`
[01:08:11]    |
[01:08:11]    = note: expected type `std::result::Result<rustc::mir::interpret::Scalar, rustc::mir::interpret::EvalError<'tcx>>`
[01:08:11]               found type `rustc::mir::interpret::Scalar`
[01:08:11]
[01:08:11] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:08:11]   --> tools/miri/src/helpers.rs:99:19
[01:08:11]    |
[01:08:11] 99 |         let raw = sign_extend(self.tcx.tcx, raw, self.tcx.types.isize)?;
[01:08:11]    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters
[01:08:11]
[01:08:11] error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
[01:08:11]   --> tools/miri/src/helpers.rs:99:19
[01:08:11]    |
[01:08:11] 99 |         let raw = sign_extend(self.tcx.tcx, raw, self.tcx.types.isize)?;
[01:08:11]    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `u128`
[01:08:11]    |
[01:08:11]    = help: the trait `std::ops::Try` is not implemented for `u128`
[01:08:11]    = note: required by `std::ops::Try::into_result`
[01:08:11]
[01:08:11] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:08:11]    --> tools/miri/src/helpers.rs:117:19
[01:08:11]     |
[01:08:11] 117 |         let raw = sign_extend(self.tcx.tcx, raw, self.tcx.types.i32)?;
[01:08:11]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters
[01:08:11]
[01:08:11] error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
[01:08:11]    --> tools/miri/src/helpers.rs:117:19
[01:08:11]     |
[01:08:11] 117 |         let raw = sign_extend(self.tcx.tcx, raw, self.tcx.types.i32)?;
[01:08:11]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `u128`
[01:08:11]     |
[01:08:11]     = help: the trait `std::ops::Try` is not implemented for `u128`
[01:08:11]     = note: required by `std::ops::Try::into_result`
[01:08:11]
[01:08:11] error[E0599]: no method named `get_local` found for type `&rustc_mir::interpret::Frame<'_, '_>` in the current scope
[01:08:11]    --> tools/miri/src/validation.rs:120:42
[01:08:11]     |
[01:08:11] 120 |                 let value = self.frame().get_local(v)?;
[01:08:11]     |                                          ^^^^^^^^^
[01:08:11]
[01:08:11] error[E0308]: mismatched types
[01:08:11]    --> tools/miri/src/validation.rs:512:33
[01:08:11]     |
[01:08:11] 512 |         self.memory.check_align(ptr, align)?;
[01:08:11]     |                                 ^^^ expected enum `rustc::mir::interpret::Scalar`, found enum `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:11]     |
[01:08:11]     = note: expected type `rustc::mir::interpret::Scalar`
[01:08:11]                found type `rustc::mir::interpret::ScalarMaybeUndef`
[01:08:11]
[01:08:11] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:11]    --> tools/miri/src/validation.rs:593:31
[01:08:11]     |
[01:08:11] 593 |                 let ptr = ptr.to_ptr()?;
[01:08:11]     |                               ^^^^^^
[01:08:11]
[01:08:11] error[E0599]: no method named `to_ptr` found for type `rustc::mir::interpret::ScalarMaybeUndef` in the current scope
[01:08:11]    --> tools/miri/src/validation.rs:682:51
[01:08:11]     |
[01:08:11] 682 |                     let ptr = self.into_ptr(ptr)?.to_ptr()?;
[01:08:11]     |                                                   ^^^^^^
[01:08:11]
[01:08:11] error: aborting due to 86 previous errors
