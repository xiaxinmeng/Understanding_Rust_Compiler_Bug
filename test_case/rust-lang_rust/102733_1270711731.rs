
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct _IMAGE_TLS_DIRECTORY64 {
    pub StartAddressOfRawData: ULONGLONG,
    pub EndAddressOfRawData: ULONGLONG,
    pub AddressOfIndex: ULONGLONG,
    pub AddressOfCallBacks: ULONGLONG,
    pub SizeOfZeroFill: DWORD,
    pub __bindgen_anon_1: _IMAGE_TLS_DIRECTORY64__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IMAGE_TLS_DIRECTORY64__bindgen_ty_1 {
    pub Characteristics: DWORD,
    pub __bindgen_anon_1: _IMAGE_TLS_DIRECTORY64__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[repr(align(4))]   /* <=== THIS LINE */
#[derive(Debug, Copy, Clone)]
pub struct _IMAGE_TLS_DIRECTORY64__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
