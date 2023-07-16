 rust
> nm --defined-only ./x*/stage1/lib/r*/x*/l*/libstd-*.so | grep -v 'expr_fn\|\.\.\|GCC\|str\|vtable' | c++filt | sort -R | head
0000000000098880 T f64::log2::h538bc4f03a4e4f8daM::v0.9.pre
0000000000111d60 T repr::Repr$int::write_repr::hffda9a03775c36faBsad::v0.9.pre
00000000000ffec0 t io::Reader::bytes::h96688705a1fe48ecam::v0.9.pre
0000000000124acc R libc::consts::os::sysconf::_SC_SYNCHRONIZED_IO::h757c915c4e1bfec5aI::v0.9.pre
00000000000b4b40 T num::FromPrimitive$u64::from_f32::hbebb3eb62bb65c6cMlaQ::v0.9.pre
00000000000b5ad0 T to_bytes::IterBytes$f32::iter_bytes::hdc01dc1017ca4c92P2ax::v0.9.pre
0000000000099b90 T f64::Exponential$f64::exp2::h165ce8970a5024afL4ay::v0.9.pre
0000000000093b70 T f32::atan::h835d81dcdb9e8aecaD::v0.9.pre
0000000000124ad8 R libc::consts::os::sysconf::_SC_MEMLOCK::h757c915c4e1bfec5aU::v0.9.pre
0000000000125cb0 R io::GroupWrite::hb29fe352308d0a41aF::v0.9.pre
