
rustc: Remove matching on ~str from the language

The `~str` type is not long for this world as it will be superseded by the
soon-to-come DST changes for the language. The new type will be
`~Str`, and matching over the allocation will no longer be supported.
Matching on `&str` will continue to work, in both a pre and post DST world.
