rust
/// Serialize a data structure asynchronously.
pub trait AsyncSerialize: Encodable
{
    /// The concrete [future](Future) returned by the `serialize` method.
    type Future<'w, F, W>: Future<Output=Result<(), F::Error>> + Unpin
    where
        Self: 'w,
        F: 'w + FormatSerialize,
        W: 'w + AsyncWrite + Unpin,
    ;

    /// Attempt to serialize the type asynchronously for the indicated [format](Format)
    /// via the provided [asynchronous writer](AsyncWrite).
    fn serialize<'w, F, W>(&'w self, format: &'w F, writer: &'w mut W) -> Self::Future<'w, F, W>
    where
        F: FormatSerialize,
        W: AsyncWrite + Unpin,
    ;
}

/// Deserialize a data structure asynchronously.
pub trait AsyncDeserialize: Decodable
{
    /// The concrete [future](Future) returned by the `deserialize` method.
    type Future<'r, F, R>: Future<Output=Result<Self, F::Error>> + Unpin
    where
        F: 'r + FormatDeserialize,
        R: 'r + AsyncRead + AsyncBufRead + Unpin,
    ;

    /// Attempt to deserialize the type asynchronously for the indicated [format](Format)
    /// via the provided [asynchronous reader](AsyncRead).
    fn deserialize<'r, F, R>(format: &'r F, reader: &'r mut R) -> Self::Future<'r, F, R>
    where
        F: FormatDeserialize,
        R: AsyncRead + AsyncBufRead + Unpin,
    ;
}
