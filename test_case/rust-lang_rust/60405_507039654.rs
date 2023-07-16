rust
union Mix {
  f1: (bool, u32),
  f2: (u32, bool),
}
let m = Mix { f1: (false, 3) };
m.f2.0 = 3;
