rust
{
    struct S;

    macro_rules! mac { () => {} }
    // `mac`'s scope starts here

    /// `mac` <- `resolve_str_path_error` won't see this
   struct Z;

    //`mac`'s scope ends here
}
