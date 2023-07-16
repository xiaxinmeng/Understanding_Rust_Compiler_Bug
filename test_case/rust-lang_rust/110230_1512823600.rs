rust
    /// Internal function to translate between an encoded span and the expanded representation.
    /// This function must not be used outside the incremental engine.
    #[inline]
    pub fn data_untracked(self) -> SpanData {
        if self.len_or_tag != LEN_TAG {
            // Inline format.
            if self.len_or_tag & PARENT_MASK == 0 {
                debug_assert!(self.len_or_tag as u32 <= MAX_LEN);
                SpanData {
                    lo: BytePos(self.base_or_index),
                    hi: BytePos(self.base_or_index + self.len_or_tag as u32),
                    ctxt: SyntaxContext::from_u32(self.ctxt_or_tag as u32),
                    parent: None,
                }
            } else {
                let len = self.len_or_tag & !PARENT_MASK;
                debug_assert!(len as u32 <= MAX_LEN);
                let parent =
                    LocalDefId { local_def_index: DefIndex::from_u32(self.ctxt_or_tag as u32) };
                SpanData {
                    lo: BytePos(self.base_or_index),
                    hi: BytePos(self.base_or_index + len as u32),
                    ctxt: SyntaxContext::root(),
                    parent: Some(parent),
                }
            }
        } else {
            // Interned format.
            let index = self.base_or_index;
            with_span_interner(|interner| interner.spans[index as usize]) // *** THIS LINE ***
        }
    }
