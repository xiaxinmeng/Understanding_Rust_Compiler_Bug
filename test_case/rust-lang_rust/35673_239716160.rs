 rust
    fn baud(&mut self) -> u32 {
        // baud is given by the following:
        // baud = 65536*(1-(samples_per_bit)*(f_wanted/f_ref))
        // samples_per_bit = 16, 8, or 3
        // f_ref = 48e6
        let wanted_f: f64 = self.baudrate as f64;
        let baud: f64 = 65536f64 * (1f64 - (16f64 * (wanted_f / 48_000000f64)));

        // https://github.com/rust-lang/rust/issues/35673
        // floor(n) = n - (n % 1)
        (baud - (baud % 1f64)) as u32
    }
