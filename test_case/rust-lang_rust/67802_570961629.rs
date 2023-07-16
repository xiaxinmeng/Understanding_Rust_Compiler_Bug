
  = note: /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `<hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::IntoIterator>::into_iter':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1878: undefined reference to `<hashbrown::raw::RawTable<T> as core::iter::traits::collect::IntoIterator>::into_iter'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `<hashbrown::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::size_hint':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1895: undefined reference to `<hashbrown::raw::RawIter<T> as core::iter::traits::iterator::Iterator>::size_hint'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1895: undefined reference to `<hashbrown::raw::RawIter<T> as core::iter::traits::iterator::Iterator>::size_hint'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `<hashbrown::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1888: undefined reference to `<hashbrown::raw::RawIter<T> as core::iter::traits::iterator::Iterator>::next'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `<hashbrown::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::size_hint':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1895: undefined reference to `<hashbrown::raw::RawIter<T> as core::iter::traits::iterator::Iterator>::size_hint'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `<hashbrown::map::IntoIter<K,V> as core::iter::traits::iterator::Iterator>::next':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1945: undefined reference to `<hashbrown::raw::RawIntoIter<T> as core::iter::traits::iterator::Iterator>::next'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::raw::RawTable<T>::find':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:882: undefined reference to `hashbrown::raw::RawTable<T>::probe_seq'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:883: undefined reference to `hashbrown::raw::RawTable<T>::ctrl'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:886: undefined reference to `hashbrown::raw::RawTable<T>::bucket'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:887: undefined reference to `hashbrown::raw::Bucket<T>::as_ref'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::reserve':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:583: undefined reference to `hashbrown::raw::RawTable<T>::reserve'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::raw::RawTable<T>::find':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:882: undefined reference to `hashbrown::raw::RawTable<T>::probe_seq'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:883: undefined reference to `hashbrown::raw::RawTable<T>::ctrl'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:886: undefined reference to `hashbrown::raw::RawTable<T>::bucket'
          /usr/sbin/ld: /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:887: undefined reference to `hashbrown::raw::Bucket<T>::as_ref'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::reserve':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:583: undefined reference to `hashbrown::raw::RawTable<T>::reserve'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::with_hasher':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:266: undefined reference to `hashbrown::raw::RawTable<T>::new'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::with_hasher':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:266: undefined reference to `hashbrown::raw::RawTable<T>::new'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::iter':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:430: undefined reference to `hashbrown::raw::RawTable<T>::iter'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1bi9i1a99w6ukhk0.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::iter':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:430: undefined reference to `hashbrown::raw::RawTable<T>::iter'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1faphbo1kfqvuvdd.rcgu.o): in function `<rustc_mir::transform::inline::Integrator as rustc::mir::visit::MutVisitor>::visit_local':
          /home/aaron/repos/rust/src/librustc_mir/transform/inline.rs:671: undefined reference to `_ZN9rustc_mir9transform6inline10Integrator20make_integrate_local17h85a5bc2e64ccfdb8E.llvm.17716994606215347223'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.1faphbo1kfqvuvdd.rcgu.o): in function `rustc::mir::visit::MutVisitor::super_statement':
          /home/aaron/repos/rust/src/librustc/mir/visit.rs:(.text._ZN5rustc3mir5visit10MutVisitor22super_basic_block_data17h231264b8007af5deE+0x71): undefined reference to `_ZN9rustc_mir9transform6inline10Integrator20make_integrate_local17h85a5bc2e64ccfdb8E.llvm.17716994606215347223'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `core::ptr::real_drop_in_place':
          /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawIntoIter<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `core::ptr::real_drop_in_place':
          /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawTable<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `core::ptr::real_drop_in_place':
          /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawTable<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::with_hasher':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:266: undefined reference to `hashbrown::raw::RawTable<T>::new'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::with_hasher':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:266: undefined reference to `hashbrown::raw::RawTable<T>::new'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `<hashbrown::map::IntoIter<K,V> as core::iter::traits::iterator::Iterator>::next':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1945: undefined reference to `<hashbrown::raw::RawIntoIter<T> as core::iter::traits::iterator::Iterator>::next'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `core::ptr::real_drop_in_place':
          /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawIntoIter<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::with_hasher':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:266: undefined reference to `hashbrown::raw::RawTable<T>::new'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `hashbrown::map::HashMap<K,V,S>::with_hasher':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:266: undefined reference to `hashbrown::raw::RawTable<T>::new'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `<hashbrown::map::IntoIter<K,V> as core::iter::traits::iterator::Iterator>::next':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/map.rs:1945: undefined reference to `<hashbrown::raw::RawIntoIter<T> as core::iter::traits::iterator::Iterator>::next'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2cct58omhch0nmrk.rcgu.o): in function `core::ptr::real_drop_in_place':
          /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawIntoIter<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2czhn8xrpxst0wlf.rcgu.o): in function `core::ptr::real_drop_in_place':
          /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawTable<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /home/aaron/repos/rust/src/libcore/ptr/mod.rs:184: undefined reference to `<hashbrown::raw::RawTable<T> as core::ops::drop::Drop>::drop'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2h900bpbh0zmqpwn.rcgu.o): in function `hashbrown::raw::RawTable<T>::buckets':
          /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.6.2/src/raw/mod.rs:919: undefined reference to `str.0.llvm.8333985234810216789'
          /usr/sbin/ld: /tmp/rustcSiRFTe/librustc_mir-73e40d5863ab96ba.rlib(rustc_mir-73e40d5863ab96ba.2h900bpbh0zmqpwn.rcgu.o): relocation R_X86_64_PC32 against undefined hidden symbol `str.0.llvm.8333985234810216789' can not be used when making a shared object
          /usr/sbin/ld: final link failed: bad value
          clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
          

error: aborting due to previous error

error: could not compile `rustc_driver`.
