rust
struct Href<'a, P>
where
    P: fmt::Display,
{
    root: &'a str,
    parent_directories: ParentDirectories,
    path_components: P,
    filename_prefix: &'a str,
    filename_base: Cow<'a, str>,
    fragment: &'a str,
}
