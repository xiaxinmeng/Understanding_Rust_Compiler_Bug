
// The VaArgSafe trait needs to be used in public interfaces, however, the trait
// itself must not be allowed to be used outside this module. Allowing users to
// implement the trait for a new type (thereby allowing the va_arg intrinsic to
// be used on a new type) is likely to cause undefined behavior.
//
// FIXME(dlrobertson): In order to use the VaArgSafe trait in a public interface
// but also ensure it cannot be used elsewhere, the trait needs to be public
// within a private module. Once RFC 2145 has been implemented look into
// improving this.
mod sealed_trait {
    /// Trait which permits the allowed types to be used with [super::VaListImpl::arg].
    #[unstable(
        feature = "c_variadic",
        reason = "the `c_variadic` feature has not been properly tested on \
                  all supported platforms",
        issue = "44930"
    )]
    pub trait VaArgSafe {}
}

