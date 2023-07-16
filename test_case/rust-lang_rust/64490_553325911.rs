rust
            let u = $crate::mem::MaybeUninit::<$Struct>::uninitialized();
            let &$Struct { $field: ref f, .. } = unsafe { &*u.as_ptr() };
