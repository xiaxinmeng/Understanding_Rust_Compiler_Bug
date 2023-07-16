rust
let mut entries = dir
    .map(|res| res.map(|e| e.path()))
    // this will return the iterator collected to a `Vec` or the first error
    .collect::<Result<Vec<_>>>()?;

println!("Entries before sorting (may or may not be sorted already): {:?}", entries);

entries.sort();

println!("Entries after sorting: {:?}", entries);
