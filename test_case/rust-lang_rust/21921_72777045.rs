
  [#bench]
  fn bench_dft_32_fft_65536(b: &mut Bencher) {
    let mut input:[Complex<f32>; 65536] = [Complex { re: 0f32, im: 0f32 }; 65536];
    for i in range(0, 65536) {
        input[i].re = ((i as f32) / 65536f32 * PI).sin();
    }
    let mut output:[Complex<f32>; 65536] = [Complex { re: 0f32, im: 0f32 }; 65536];
    b.iter(|| {
      unsafe {
        let mut operator = fft_f32(65536).ok().unwrap();
        operator.fft(&input[0] as *const Complex<f32>, &mut output[0] as *mut Complex<f32>, 1);
      }
    });
  }
