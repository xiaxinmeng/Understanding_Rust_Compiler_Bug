rust
  loop {
      let [dst1, dst2, src1] = slice.get_many_mut([i, j, k]).unwrap();
      if pred(src) {
          *dst1 = *src1
          i += 1;
      } else {
          *dst2 = *src1
          j += 1;
      }
      k += 1;
  }
  