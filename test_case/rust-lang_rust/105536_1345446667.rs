rust
                debug!("ScalarPair(a: {:?}, b: {:?})", a, b);
                // We know `offset` is relative to the allocation, so we can use `into_parts`.
                let (data, start) = match a.to_pointer(ecx).unwrap().into_parts() {
                    (Some(alloc_id), offset) => {
                        (ecx.tcx.global_alloc(alloc_id).unwrap_memory(), offset.bytes())
                    }
                    (None, _offset) => (
                        ecx.tcx.intern_const_alloc(Allocation::from_bytes_byte_aligned_immutable(
                            b"" as &[u8],
                        )),
                        0,
                    ),
                };
                let len = b.to_machine_usize(ecx).unwrap();
                let start = start.try_into().unwrap();
                let len: usize = len.try_into().unwrap();
                ConstValue::Slice { data, start, end: start + len }
