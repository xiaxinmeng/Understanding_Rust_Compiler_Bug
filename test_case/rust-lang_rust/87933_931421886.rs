rust
    #[used]
    #[link_section = "__TEXT,__asar_integrity"]
    #[no_mangle]
    static _EMBED_ASAR_INTEGRITY: [u8; 4] = *b"1234";

    extern "C" {
        #[link_name = "_EMBED_ASAR_INTEGRITY"]
        static START: [u8; 0];
    }
