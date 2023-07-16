rust
impl<T> AsVertexFormat for T
where
    T: Attribute,
{
    // i'd say the implied 'static on that slice is suspect, but it's actually explicit in the const declaration >_>
    const VERTEX_FORMAT: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            Element {
                format: T::SELF,
                offset: 0,
            },
        ]),
        stride: T::SIZE,
    };
}
