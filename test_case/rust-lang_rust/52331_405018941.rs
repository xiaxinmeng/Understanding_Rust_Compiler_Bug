rust
impl<'a> PrefixComponent<'a> {
    /// On Windows, returns the parsed prefix data.
    ///
    /// On other platforms, the returned value is unspecified.
    #[cfg_attr(
        target_os = "redox",
        rustc_deprecated(
            since = "1.29.0", 
            reason = "The prefix component is meaningless on Redox. Use `.scheme()` instead",
        ),
    )]
    pub fn kind(&self) -> Prefix<'a> { ... }
}

mod redox {
    pub trait PrefixComponentExt<'a> {
        // Obtains the scheme of the Redox path (URL).
        fn scheme(&self) -> &'a OsStr;
    }

    impl<'a> PrefixComponentExt<'a> for PrefixComponent<'a> {
        fn scheme(&self) -> &'a OsStr { self.as_os_str() }
    }
}
