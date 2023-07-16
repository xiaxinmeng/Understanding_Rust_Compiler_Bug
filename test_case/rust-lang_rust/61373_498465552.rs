rust
if pred { StorageLive(a); } else { StorageLive(b); }
// ...later...
if pred { a = 42; } else { b = 99; }
