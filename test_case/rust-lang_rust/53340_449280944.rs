rust
fn decode_12le(buf: &[u8], width: usize, height: usize) -> Vec<u16> {
  let mut out: Vec<u16> = vec![0; width*height];

  for (row, line) in out.chunks_exact_mut(width).enumerate() {
    let inb = &buf[(row*width*12/8)..];

    for (o, i) in line.chunks_exact_mut(2).zip(inb.chunks(3)) {
      let g1: u16 = i[0] as u16;
      let g2: u16 = i[1] as u16;
      let g3: u16 = i[2] as u16;

      o[0] = ((g2 & 0x0f) << 8) | g1;
      o[1] = (g3 << 4) | (g2 >> 4);
    }
  }
  out
}

fn main() {
  let width = 5000;
  let height = 4000;

  let mut buffer: Vec<u8> = vec![0; width*height*12/8];
  // Make sure we don't get optimized out by writing some data into the buffer
  for (i, val) in buffer.chunks_mut(1).enumerate() {
    val[0] = i as u8;
  }
  
  for _ in 0..100 {
    decode_12le(&buffer, width, height);
  }
}
