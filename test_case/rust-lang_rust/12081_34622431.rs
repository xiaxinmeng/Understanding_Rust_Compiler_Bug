 rust

             if table::hash_of(&idx) == hash {
                 let (bucket_k, _) = self.table.read_mut(&idx);
                 if k == *bucket_k {
                     // Key already exists. Get its reference.
                     let (_, bucket_v) = self.table.read_mut(&idx);
                     return bucket_v;
                 }
             }
