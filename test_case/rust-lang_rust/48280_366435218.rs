rust
#[repr(C)]
struct FtGlyphSlot {
    library: *mut FtLibrary,
    face: *mut FtFace,
    next: *mut FtGlyphSlot,
    reserved: libc::c_uint,
    generic: FtGeneric,

    metrics: FtGlyphMetrics,
    linear_hori_advance: FtFixed,
    linear_vert_advance: FtFixed,
    advance: FtVector,

    format: FtGlyphFormat,

    bitmap: FtBitmap,
    bitmap_left: libc::c_int,
    bitmap_top: libc::c_int,

    outline: FtOutline,

    num_subglyphs: libc::c_uint,
    subglyphs: FtSubGlyph,

    control_data: *mut libc::c_void,
    control_len: libc::c_long,

    lsb_delta: FtPos,
    rsb_delta: FtPos,

    other: *mut libc::c_void,

    internal: *mut FtSlotInternal,
}
