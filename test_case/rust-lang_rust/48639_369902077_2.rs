rust
   // Note: We should also permute the original vector at the end.
   let mut indices: Vec<_> = v.iter()
       .map(|x| Reverse(*x))
       .enumerate()
       .map(|(i, k)| (k, i))
       .collect();
   indices.sort_unstable();
   