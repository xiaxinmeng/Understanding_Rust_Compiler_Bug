rust
pub existential type GetResponse: Future<Output = crate::Result<crate::Mod>> + 'static;
pub existential type DownloadResponse: Stream<Item = crate::Result<crate::reqwest_async::Chunk>> + 'static;
