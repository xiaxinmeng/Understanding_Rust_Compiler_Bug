rust
let compatibility_matrix = (0..provided_count)
            .map(|i| {
                (0..expected_input_count)
                    .map(|j| is_compatible(ProvidedIdx::from_usize(i), ExpectedIdx::from_usize(j)))
                    .collect()
            })
            .collect();
        ArgMatrix {
            provided_indices: (0..provided_count).map(ProvidedIdx::from_usize).collect(),
            expected_indices: (0..expected_input_count).map(ExpectedIdx::from_usize).collect(),
            compatibility_matrix,
        }
