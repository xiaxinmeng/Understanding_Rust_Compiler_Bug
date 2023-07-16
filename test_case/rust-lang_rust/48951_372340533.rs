rust
let lines = BufReader::new(file).lines();

// (note: the author's intent here was to bail on Err.  Perhaps this was simply
//  done without much thought, by copying from the first line of the for loop)
let first = lines.next()?;

match first.split_whitespace().next() {
    ...
}

for line in lines {
    let line = line?;
    ...
}
