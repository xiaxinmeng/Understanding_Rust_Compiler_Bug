
// in metadata
trait MetadataStore {
    fn get_metadata(&self) -> Result<MetadataBlob, String>;
    ...
}

// in trans or somewhere else
impl MetadataStore for LlvmMetadataStore { ... }

// in driver
session.metadata_store_impl: Box<MetadataStore> = Box::new(LlvmMetadataStore);  // or something like that
