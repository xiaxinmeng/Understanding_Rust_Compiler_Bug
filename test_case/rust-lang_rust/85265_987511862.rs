rust
                // Pass and return structures up to 2 pointers in size by value, matching `ScalarPair`.
                // LLVM will usually pass these in 2 registers, which is more efficient than by-ref.
                let max_by_val_size = Pointer.size(self) * 2;
                let size = arg.layout.size;

                if arg.layout.is_unsized() || size > max_by_val_size {
                    arg.make_indirect();
                } else {
                    // We want to pass small aggregates as immediates, but using
                    // a LLVM aggregate type for this leads to bad optimizations,
                    // so we pick an appropriately sized integer type instead.
                    arg.cast_to(Reg { kind: RegKind::Integer, size });
                }

