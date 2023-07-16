
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `&dyn Tr1<As1 = impl for<'a> Tr2<'a>> + std::marker::Sync`,
 right: `&dyn Tr1<As1 = cdef_et4::A> + std::marker::Sync`: mismatch of cast type &dyn Tr1<As1 = impl for<'a> Tr2<'a>> + std::marker::Sync and place type &dyn Tr1<As1 = cdef_et4::A> + std::marker::Sync', src/librustc_mir/interpret/cast.rs:28:17
