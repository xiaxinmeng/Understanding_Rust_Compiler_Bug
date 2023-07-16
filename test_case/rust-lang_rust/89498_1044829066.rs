rust
        if abi == SpecAbi::Rust
            || abi == SpecAbi::RustCall
            || abi == SpecAbi::RustIntrinsic
            || abi == SpecAbi::PlatformIntrinsic
        {
            let fixup = |arg: &mut ArgAbi<'tcx, Ty<'tcx>>| {
                if arg.is_ignore() {
                    return;
                }

                match arg.layout.abi {
                    Abi::Aggregate { .. } => {}

                    // This is a fun case! The gist of what this is doing is
                    // that we want callers and callees to always agree on the
                    // ABI of how they pass SIMD arguments. If we were to *not*
                    // make these arguments indirect then they'd be immediates
                    // in LLVM, which means that they'd used whatever the
                    // appropriate ABI is for the callee and the caller. That
                    // means, for example, if the caller doesn't have AVX
                    // enabled but the callee does, then passing an AVX argument
                    // across this boundary would cause corrupt data to show up.
                    //
                    // This problem is fixed by unconditionally passing SIMD
                    // arguments through memory between callers and callees
                    // which should get them all to agree on ABI regardless of
                    // target feature sets. Some more information about this
                    // issue can be found in #44367.
                    //
                    // Note that the platform intrinsic ABI is exempt here as
                    // that's how we connect up to LLVM and it's unstable
                    // anyway, we control all calls to it in libstd.
                    Abi::Vector { .. }
                        if abi != SpecAbi::PlatformIntrinsic
                            && self.tcx.sess.target.simd_types_indirect =>
                    {
                        arg.make_indirect();
                        return;
                    }

                    _ => return,
                }

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
            };
            fixup(&mut fn_abi.ret);
            for arg in &mut fn_abi.args {
                fixup(arg);
            }
        } else {
            fn_abi.adjust_for_foreign_abi(self, abi)?;
        }
