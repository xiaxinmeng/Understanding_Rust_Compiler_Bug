
pub struct PathSegment<'hir> {
    pub ident: Ident,      // 12 bytes
    pub hir_id: HirId,     // 8 bytes
    pub res: Res,          // 12 bytes
    pub args: Option<&'hir GenericArgs<'hir>>, // 8 bytes
    pub infer_args: bool,  // 1 bit of data taking up 1 byte, followed by 7 bytes of padding
}
