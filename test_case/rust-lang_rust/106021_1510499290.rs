
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive] // May want to add the tool list?
pub struct MachOBuildVersion {
    /// One of the `PLATFORM_` constants (for example,
    /// [`object::macho::PLATFORM_MACOS`](macho::PLATFORM_MACOS)).
    pub platform: u32,
    /// The minimum OS version, where `X.Y.Z` is encoded in nibbles as
    /// `xxxx.yy.zz`.
    pub minos: u32,
    /// The SDK version as `X.Y.Z`, where `X.Y.Z` is encoded in nibbles as
    /// `xxxx.yy.zz`.
    pub sdk: u32,
}
