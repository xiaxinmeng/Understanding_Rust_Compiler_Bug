rust
let bytes: [u8; $N] = if cfg!(target_endian = "little") {
    transmute::<_, [u8; _]>(
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(not(target_endian = "little"))]
        {
            x.swap_bytes()
        }
    )
} else {
    transmute::<_, [u8; _]>(
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(not(target_endian = "big"))]
        {
            x.swap_bytes()
        }
    )
};
