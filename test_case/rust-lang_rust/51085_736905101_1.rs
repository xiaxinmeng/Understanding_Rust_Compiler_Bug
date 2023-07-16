rust
union Uninit<T> {
  value: ManuallyDrop<T>,
  uninit: (),
}

fn main() {
  unsafe {
    let mut x: Uninit<(u32, !)> = Uninit { uninit: () };
    (*x.value).0 = 22; // Initialize first part of the tuple
    match *x.value {
      (v, _) => { // This is reachable and apparently not UB
        println!("{}", v);
      }
    }
  }
}
