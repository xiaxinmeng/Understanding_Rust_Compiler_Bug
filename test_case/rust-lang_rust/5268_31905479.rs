 rust
let left_over;
loop {
     let first = match me.next() {
          None => { left_over = None; break }
          Some(x) => x
     });
     let second = match me.next() {
          None => { left_over = Some(first); break }
          Some(x) => x
     };

     // do the comparisons to adjust min and max
}

match left_over {
    None = > MinMax(min, max)
    Some(x) => {
        // do the comparisons for the trailing element
    }
}
