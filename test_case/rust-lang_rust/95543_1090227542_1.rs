rust
    impl<K: DepKind> SerializedDepGraph<K> {

        #[inline]
        fn decoder_at(&self, dep_node_index: SerializedDepNodeIndex) -> opaque::Decoder<'_> {
            let address = self.node_addresses[dep_node_index];
            opaque::Decoder::new(self.node_data, address)
        }

        #[inline]
        pub fn fingerprint_by_index(&self, dep_node_index: SerializedDepNodeIndex) -> Fingerprint {
            Fingerprint::decode(&mut self.decoder_at(dep_node_index))
        }

        #[inline]
        pub fn edge_targets_from(&self, source: SerializedDepNodeIndex) -> SerializedDepNodeIndexIter {
            let mut decoder = self.decoder_at(source);
            // Skip the fingerprint
            decoder.read_raw_bytes(SIZE_OF_ENCODED_FINGERPRINT);

            SerializedDepNodeIndexIter::new(decoder)
        }
    }

    struct SerializedDepNodeIndexIter<'a> {
        decoder: opaque::Decoder<'a>,
        entry_size: usize,
        entries_left: usize,
    }

    impl SerializedDepNodeIndexIter<'a> {

        fn new(decoder: opaque::Decoder<'a>,) -> Self {
            let entry_count_and_size = decoder.read_u32();
            let entries_left = entry_count_and_size >> 2;

            // obviously this would not really need to be implemented with `match` :)
            let entry_size = match entry_count_and_size & 0b11 {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 4,
            };

            Self {
                decoder,
                entry_size,
                entries_left,
            }
        }
    }

    impl Iterator for SerializedDepNodeIndexIter {
        type Item = SerializedDepNodeIndex;

        fn next(&mut self) -> Option<SerializedDepNodeIndex> {
            if self.entries_left == 0 {
                return None;
            }

            self.entries_left -= 1;

            return match self.entry_size  {
                // Read 1, 2, 3, or 4 bytes and convert to SerializedDepNodeIndex
            };
        }
    }
    