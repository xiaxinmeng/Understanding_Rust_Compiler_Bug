rs
                let mut bx = $crate::boxed::Box::new_uninit();
                ::core::intrinsics::write_via_move(bx.as_mut_ptr(), [$($x),+]);
                bx.assume_init()
