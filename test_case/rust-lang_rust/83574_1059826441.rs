rust
for (((a, b), c), d) in ia.zip(ib).zip(ic).zip(id) {
   // ...
}

for (a, b, c, d) in iter::zip!(ia, ib, ic, id) {
   // ...
}
