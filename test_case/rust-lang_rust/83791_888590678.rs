rs
let mut indices = 0..;
let iter1 = …; // some finite iterator
let iter2 = …; // some finite iterator
let iter3 = …; // some finite iterator
for (item, index) in iter1.zip(&mut indices) {
    // do something
}
for (item, index) in iter2.zip(&mut indices) {
    // do something, assumes no index was skipped from the last index in the previous loop
}
for (item, index) in iter3.zip(&mut indices) {
    // do something, assumes no index was skipped from the last index in the previous loop
}
