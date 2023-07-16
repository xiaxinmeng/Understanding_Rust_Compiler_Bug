 rust
            self.freqs[FREQ[(CLASS[buf[0] as uint] << 3 | CLASS[buf[1] as uint]) as uint] as uint] += 1;
            self.freqs[FREQ[(CLASS[buf[1] as uint] << 3 | CLASS[buf[2] as uint]) as uint] as uint] += 1;
            self.freqs[FREQ[(CLASS[buf[2] as uint] << 3 | CLASS[buf[3] as uint]) as uint] as uint] += 1;
            self.freqs[FREQ[(CLASS[buf[3] as uint] << 3 | CLASS[buf[4] as uint]) as uint] as uint] += 1;
            self.freqs[FREQ[(CLASS[buf[4] as uint] << 3 | CLASS[buf[5] as uint]) as uint] as uint] += 1;
            self.freqs[FREQ[(CLASS[buf[5] as uint] << 3 | CLASS[buf[6] as uint]) as uint] as uint] += 1;
            self.freqs[FREQ[(CLASS[buf[6] as uint] << 3 | CLASS[buf[7] as uint]) as uint] as uint] += 1;
