 rust
          if size_of::<$T>()*8 == 128 {
            let b = x.log2()? / (base.log2()?+1);
            n += b;
            x /= base.pow(b as u32);
          }
