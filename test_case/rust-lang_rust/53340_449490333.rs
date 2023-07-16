rust
      let g3: u16 = i[2] as u16;
      let g2: u16 = i[1] as u16;
      let g1: u16 = i[0] as u16;

      o[1] = (g3 << 4) | (g2 >> 4);
      o[0] = ((g2 & 0x0f) << 8) | g1;
