
thread 'rustc' panicked at 'Layout mismatch when copying!
src: OpTy {
    op: Immediate(
        Scalar(
            0x00000000,
        ),
    ),
    layout: TyLayout {
        ty: i32,
        details: <lots of stuff>
    }
}
dest: PlaceTy {
    place: Ptr(
        MemPlace {
            ptr: alloc0+0,
            align: Align {
                pow2: 3,
            },
            meta: None,
        },
    ),
    layout: TyLayout {
        ty: usize,
        details: <lots of stuff>
    }
}
